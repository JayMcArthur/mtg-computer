use mtg_computer::{
    SYMBOL_CARDS,
    compiler::compile,
    machine::{Machine, describe},
};
use std::{env, fs, process::ExitCode};

fn execute() -> Result<(), String> {
    let args: Vec<_> = env::args().skip(1).collect();
    let usage = "usage: mtg-computer <compile|run|trace> FILE [MAX_STEPS]\n       mtg-computer fixture [SOURCE]\n       mtg-computer board SOURCE OUTPUT_DIR [MAX_BOUNDARIES]\n       mtg-computer replay TRACE OUTPUT.html\ntrace emits JSON Lines; run/trace default to 10000 instructions";
    if (3..=4).contains(&args.len()) && args[0] == "board" {
        let source = fs::read_to_string(&args[1]).map_err(|e| format!("{}: {e}", args[1]))?;
        let limit = args
            .get(3)
            .map(|s| {
                s.parse::<usize>()
                    .map_err(|_| "MAX_BOUNDARIES must be a positive integer".to_owned())
            })
            .transpose()?
            .unwrap_or(10_000);
        let html =
            mtg_computer::backend::run(&compile(&source)?, std::path::Path::new(&args[2]), limit)?;
        println!(
            "verified Magic execution; offline replay: {}",
            html.display()
        );
        return Ok(());
    }
    if args.len() == 3 && args[0] == "replay" {
        let trace = fs::read_to_string(&args[1]).map_err(|e| format!("{}: {e}", args[1]))?;
        let html = mtg_computer::replay::render(&trace)?;
        fs::write(&args[2], html).map_err(|e| format!("{}: {e}", args[2]))?;
        println!("wrote offline replay to {}", args[2]);
        return Ok(());
    }
    if args.first().map(String::as_str) == Some("fixture") && args.len() <= 2 {
        let fixture = if args.len() == 2 {
            let source = fs::read_to_string(&args[1]).map_err(|e| format!("{}: {e}", args[1]))?;
            mtg_computer::construction::prepare_program(&compile(&source)?)?
        } else {
            mtg_computer::construction::add_halt_fixture()
        };
        println!(
            "{}",
            serde_json::to_string_pretty(&fixture).map_err(|e| e.to_string())?
        );
        return Ok(());
    }
    if !(2..=3).contains(&args.len()) || !matches!(args[0].as_str(), "compile" | "run" | "trace") {
        return Err(usage.into());
    }
    let source = fs::read_to_string(&args[1]).map_err(|e| format!("{}: {e}", args[1]))?;
    let program = compile(&source)?;
    if args[0] == "compile" {
        if args.len() != 2 {
            return Err(usage.into());
        }
        for (name, r) in &program.variables {
            println!("{name} = r{r} (Joraga Warcaller +1/+1 counters)");
        }
        for (pc, instruction) in program.instructions.iter().enumerate() {
            let [x, y, z] = instruction.symbols;
            let cards = instruction.symbols.map(|s| SYMBOL_CARDS[s as usize]);
            println!(
                "{pc:04} | line {:3}: {}\n     | {x} {y} {z}: {}\n     | {}",
                instruction.source_line,
                program.source[instruction.source_line - 1].trim(),
                describe(instruction.symbols),
                cards.join(" / ")
            );
        }
        println!(
            "{} instructions; {} program cards (support/setup cards additional)",
            program.instructions.len(),
            3 * program.instructions.len()
        );
        return Ok(());
    }
    let limit = args
        .get(2)
        .map(|s| {
            s.parse::<usize>()
                .map_err(|_| "MAX_STEPS must be a nonnegative integer".to_owned())
        })
        .transpose()?
        .unwrap_or(10_000);
    let mut machine = Machine::default();
    machine.registers = program.initial_registers.clone();
    for _ in 0..limit {
        let frame = machine.step(&program.instructions)?;
        if args[0] == "trace" {
            println!(
                "{}",
                serde_json::to_string(&frame).map_err(|e| e.to_string())?
            );
        } else {
            let values = program
                .variables
                .iter()
                .map(|(name, r)| format!("{name}={}", frame.registers_after[*r]))
                .collect::<Vec<_>>()
                .join(" ");
            println!(
                "step {:4} | pc {:04} | source line {} | {} | {values} | flag={}",
                frame.step, frame.pc, frame.source_line, frame.instruction, frame.flag_after
            );
        }
        if machine.halted {
            return Ok(());
        }
    }
    Err(format!(
        "stopped after {limit} instructions without halting (execution limit; not a claim of nontermination)"
    ))
}

fn main() -> ExitCode {
    match execute() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}
