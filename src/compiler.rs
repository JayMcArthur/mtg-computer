use num_bigint::BigUint;
use std::collections::BTreeMap;

#[derive(Clone, Debug)]
pub struct Instruction {
    pub symbols: [u8; 3],
    pub source_line: usize,
}

#[derive(Debug)]
pub struct Program {
    pub source: Vec<String>,
    pub variables: BTreeMap<String, usize>,
    pub instructions: Vec<Instruction>,
    pub initial_registers: [BigUint; 12],
}

// r0 belongs to NumBuild; source variables occupy r1..r11.
fn register(name: &str, variables: &BTreeMap<String, usize>) -> Result<u8, String> {
    variables
        .get(name)
        .map(|r| *r as u8)
        .ok_or_else(|| format!("unknown variable '{name}'"))
}

fn identifier(name: &str) -> bool {
    let mut chars = name.chars();
    chars
        .next()
        .is_some_and(|c| c.is_ascii_alphabetic() || c == '_')
        && chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

pub fn compile(source: &str) -> Result<Program, String> {
    let mut program = Program {
        source: source.lines().map(str::to_owned).collect(),
        variables: BTreeMap::new(),
        instructions: Vec::new(),
        initial_registers: std::array::from_fn(|_| BigUint::default()),
    };
    let mut labels = BTreeMap::new();
    let mut branches = Vec::new();
    for (index, line) in source.lines().enumerate() {
        let source_line = index + 1;
        let text = line.split('#').next().unwrap().trim();
        if text.is_empty() {
            continue;
        }
        if let Some(label) = text.strip_suffix(':') {
            if !identifier(label)
                || labels
                    .insert(label.to_owned(), program.instructions.len())
                    .is_some()
            {
                return Err(format!(
                    "line {source_line}: invalid or repeated label '{label}'"
                ));
            }
            continue;
        }
        let parts: Vec<_> = text.split_whitespace().collect();
        let pc = program.instructions.len();
        let mut emitted = Vec::new();
        let mut emit = |symbols| {
            emitted.push(Instruction {
                symbols,
                source_line,
            })
        };
        let result: Result<(), String> = (|| {
            match parts.as_slice() {
                ["input", name, "=", value] => {
                    if !identifier(name) || program.variables.contains_key(*name) {
                        return Err(format!("input name '{name}' must be valid and unused"));
                    }
                    let number = value.parse::<BigUint>().map_err(|_| "input must be a natural number".to_owned())?;
                    let r = program.variables.len() + 1;
                    if r > 11 { return Err("at most 11 named variables are supported".into()); }
                    program.variables.insert((*name).to_owned(), r);
                    program.initial_registers[r] = number;
                }
                ["halt"] => emit([4, 0, 0]),
                ["goto", label] => {
                    branches.push((pc, (*label).to_owned(), false));
                    emit([3, 0, 1]);
                }
                ["if", "underflow", "goto", label] => {
                    branches.push((pc, (*label).to_owned(), true));
                    emit([3, 4, 1]);
                }
                ["if", name, "==", "0", "goto", label] => {
                    let r = register(name, &program.variables)?;
                    emit([11, r, r]);
                    branches.push((pc + 1, (*label).to_owned(), true));
                    emit([3, 4, 1]);
                }
                [name, "=", value] => {
                    if !identifier(name) { return Err(format!("invalid variable name '{name}'")); }
                    // Resolve the RHS before introducing the destination.
                    let literal = value.parse::<BigUint>().ok();
                    let rhs = if literal.is_none() { Some(register(value, &program.variables)?) } else { None };
                    let next_register = program.variables.len() + 1;
                    let r = if let Some(r) = program.variables.get(*name) { *r as u8 } else {
                        if next_register > 11 { return Err("at most 11 named variables are supported".into()); }
                        program.variables.insert((*name).to_owned(), next_register);
                        next_register as u8
                    };
                    if let Some(number) = literal {
                        for digit in number.to_radix_be(144) { emit([10, digit / 12, digit % 12]); }
                        emit([5, r, 0]);
                    } else if rhs != Some(r) {
                        emit([5, r, rhs.unwrap()]);
                    }
                    // Self-assignment emits nothing: 5 Y Y means Zero, not copy.
                }
                [name, "+=", value] => emit([0, register(name, &program.variables)?, register(value, &program.variables)?]),
                [name, "*=", value] => emit([6, register(value, &program.variables)?, register(name, &program.variables)?]),
                [name, "-=", "1"] => { let r = register(name, &program.variables)?; emit([1, r, r]); }
                _ => return Err("expected input NAME = VALUE, assignment, +=, *=, -= 1, label, goto, if NAME == 0 goto LABEL, if underflow goto LABEL, or halt (separate tokens with spaces)".into()),
            }
            Ok(())
        })();
        result.map_err(|e| format!("line {source_line}: {e}"))?;
        program.instructions.extend(emitted);
    }
    if program.instructions.is_empty() {
        return Err("program contains no instructions".into());
    }
    // Start with an upper bound on branch width, then shrink to immediate jumps
    // or the required number of base-144 digits. Shrinking intervening branches
    // can only shorten a distance, so relocation converges.
    let mut width = 1usize;
    while BigUint::from(program.instructions.len() + branches.len() * width)
        >= BigUint::from(144u32).pow(width as u32)
    {
        width += 1;
    }
    let branch_addresses: BTreeMap<_, _> = branches
        .iter()
        .map(|(pc, label, conditional)| (*pc, (label, *conditional)))
        .collect();
    let mut widths = vec![1; program.instructions.len()];
    for pc in branch_addresses.keys() {
        widths[*pc] = width + 1;
    }
    for (pc, label, _) in &branches {
        if labels
            .get(label)
            .is_none_or(|target| *target >= program.instructions.len())
        {
            return Err(format!(
                "line {}: label '{label}' must point to an instruction",
                program.instructions[*pc].source_line
            ));
        }
    }
    let relocated = loop {
        let addresses: Vec<_> = std::iter::once(0)
            .chain(widths.iter().scan(0, |address, width| {
                *address += width;
                Some(*address)
            }))
            .collect();
        let mut changed = false;
        for (pc, label, _) in &branches {
            let target = labels[label];
            let distance = addresses[target].abs_diff(addresses[*pc] + widths[*pc]);
            let immediate = if target <= *pc {
                addresses[*pc] + 1 - addresses[target]
            } else {
                distance
            };
            let needed = if (1..=11).contains(&immediate) {
                1
            } else {
                BigUint::from(distance).to_radix_be(144).len() + 1
            };
            if needed < widths[*pc] {
                widths[*pc] = needed;
                changed = true;
            }
        }
        if !changed {
            break addresses;
        }
    };
    let mut resolved = BTreeMap::new();
    for (pc, label, conditional) in branches {
        let target = labels[&label];
        // NumBuilds precede the jump; the paper advances past the jump BEFORE
        // applying the distance. Z=0 reads that distance from scratch r0.
        let after_jump = relocated[pc] + widths[pc];
        let backwards = relocated[target] < after_jump;
        let distance = relocated[target].abs_diff(after_jump);
        let mode = match (conditional, backwards) {
            (false, false) => 0,
            (false, true) => 1,
            (true, false) => 4,
            (true, true) => 5,
        };
        resolved.insert(pc, (mode, distance));
    }
    let mut code = Vec::with_capacity(*relocated.last().unwrap());
    for (pc, instruction) in program.instructions.into_iter().enumerate() {
        if let Some((mode, distance)) = resolved.get(&pc) {
            if widths[pc] == 1 {
                code.push(Instruction {
                    symbols: [3, *mode, *distance as u8],
                    source_line: instruction.source_line,
                });
                continue;
            }
            let digits = BigUint::from(*distance).to_radix_be(144);
            for digit in digits {
                code.push(Instruction {
                    symbols: [10, digit / 12, digit % 12],
                    source_line: instruction.source_line,
                });
            }
            code.push(Instruction {
                symbols: [3, *mode, 0],
                source_line: instruction.source_line,
            });
        } else {
            code.push(instruction);
        }
    }
    program.instructions = code;
    Ok(program)
}
