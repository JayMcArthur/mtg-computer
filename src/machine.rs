use crate::SYMBOL_CARDS;
use crate::compiler::Instruction;
use num_bigint::BigUint;
use serde::Serialize;

#[derive(Debug, Default)]
pub struct Machine {
    pub registers: [BigUint; 12],
    pub pc: usize,
    pub flag: bool,
    pub halted: bool,
    last_num_build: bool,
    flag_combining: bool,
    steps: usize,
}

#[derive(Debug, Serialize)]
pub struct Frame {
    pub step: usize,
    pub pc: usize,
    pub next_pc: usize,
    pub source_line: usize,
    pub symbols: [u8; 3],
    pub cards: [&'static str; 3],
    pub instruction: String,
    pub registers_before: [String; 12],
    pub registers_after: [String; 12],
    pub flag_before: bool,
    pub flag_after: bool,
    pub halted: bool,
}

pub fn describe([x, y, z]: [u8; 3]) -> String {
    match x {
        0 => format!("Add r{y} r{z}"),
        1 if y == z => format!("Sub1Cond r{z}"),
        3 => format!(
            "{} {}",
            match y {
                0 => "JumpFwd",
                1 => "JumpBwd",
                4 => "JumpFwdF",
                5 => "JumpBwdF",
                _ => "UnsupportedJump",
            },
            if z == 0 { "r0".into() } else { z.to_string() }
        ),
        4 if y == 0 && z == 0 => "HaltD".into(),
        5 if y == z => format!("Zero r{y}"),
        5 => format!("Set r{y} r{z}"),
        6 => format!("Mult r{z} r{y}"),
        10 => format!("NumBuild {}", 12 * y as u16 + z as u16),
        11 if y == z => format!("FIsZero r{z}"),
        _ => format!("unsupported {x} {y} {z}"),
    }
}

impl Machine {
    pub fn step(&mut self, code: &[Instruction]) -> Result<Frame, String> {
        if self.halted {
            return Err("machine has halted".into());
        }
        let instruction = code
            .get(self.pc)
            .ok_or("program counter is outside the program")?;
        let [x, y, z] = instruction.symbols;
        if x > 11 || y > 11 || z > 11 {
            return Err("symbols must be in 0..12".into());
        }
        let pc = self.pc;
        let registers_before = self.registers.each_ref().map(ToString::to_string);
        let flag_before = self.flag;
        let mut next_pc = (pc + 1) % code.len();
        let y = y as usize;
        let z = z as usize;
        match x {
            0 => self.registers[y] = &self.registers[y] + &self.registers[z],
            1 if y == z => {
                let underflow = self.registers[z] == BigUint::default();
                if !underflow {
                    self.registers[z] -= 1u32;
                }
                self.set_flag(underflow);
            }
            3 if matches!(y, 0 | 1 | 4 | 5) => {
                let taken = y < 4 || self.flag;
                self.flag_combining = false;
                if taken {
                    let distance = if z == 0 {
                        self.registers[0].clone()
                    } else {
                        BigUint::from(z)
                    };
                    // Backward jumps beyond the program length do nothing in the paper.
                    let backwards = y == 1 || y == 5;
                    let cycle_length = code.len().max(2);
                    if !backwards || distance <= BigUint::from(cycle_length) {
                        let distance = (&distance % BigUint::from(code.len()))
                            .to_u64_digits()
                            .first()
                            .copied()
                            .unwrap_or(0) as usize;
                        next_pc = if backwards {
                            (next_pc + code.len() - distance) % code.len()
                        } else {
                            (next_pc + distance) % code.len()
                        };
                    }
                }
            }
            4 if y == 0 && z == 0 => self.halted = true,
            5 => {
                self.registers[y] = if y == z {
                    BigUint::default()
                } else {
                    self.registers[z].clone()
                }
            }
            6 => self.registers[z] = &self.registers[z] * &self.registers[y],
            10 => {
                let digit = BigUint::from(12 * y + z);
                self.registers[0] = if self.last_num_build {
                    &self.registers[0] * 144u32 + digit
                } else {
                    digit
                };
            }
            11 if y == z => {
                self.set_flag(self.registers[z] == BigUint::default());
                self.flag_combining = true;
            }
            _ => return Err(format!("unsupported opcode: {x} {y} {z}")),
        }
        self.last_num_build = x == 10;
        self.pc = next_pc;
        self.steps += 1;
        Ok(Frame {
            step: self.steps,
            pc,
            next_pc,
            source_line: instruction.source_line,
            symbols: instruction.symbols,
            cards: instruction.symbols.map(|s| SYMBOL_CARDS[s as usize]),
            instruction: describe(instruction.symbols),
            registers_before,
            registers_after: self.registers.each_ref().map(ToString::to_string),
            flag_before,
            flag_after: self.flag,
            halted: self.halted,
        })
    }

    fn set_flag(&mut self, value: bool) {
        self.flag = if self.flag_combining {
            self.flag || value
        } else {
            value
        };
    }
}
