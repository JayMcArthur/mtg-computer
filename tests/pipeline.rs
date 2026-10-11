use mtg_computer::{
    compiler::{Instruction, compile},
    machine::Machine,
};
use num_bigint::BigUint;

fn execute(source: &str) -> Machine {
    let program = compile(source).unwrap();
    let mut machine = Machine::default();
    machine.registers = program.initial_registers.clone();
    for _ in 0..10_000 {
        machine.step(&program.instructions).unwrap();
        if machine.halted {
            return machine;
        }
    }
    panic!("test program exceeded its execution limit");
}

#[test]
fn published_cube_sample() {
    let symbols = [[10, 0, 10], [5, 1, 0], [6, 0, 1], [6, 0, 1], [4, 0, 0]];
    let code: Vec<_> = symbols
        .into_iter()
        .map(|symbols| Instruction {
            symbols,
            source_line: 1,
        })
        .collect();
    let mut machine = Machine::default();
    let mut values = Vec::new();
    for _ in 0..code.len() {
        let frame = machine.step(&code).unwrap();
        values.push(frame.registers_after[1].clone());
    }
    assert_eq!(values, ["0", "10", "100", "1000", "1000"]);
    assert!(machine.halted);
}

#[test]
fn readable_loop_matches_arithmetic_for_many_inputs() {
    for n in 0u32..25 {
        let source =
            include_str!("../examples/sum.mtg").replace("count = 10", &format!("count = {n}"));
        let program = compile(&source).unwrap();
        let result = execute(&source);
        assert_eq!(
            result.registers[program.variables["total"]],
            BigUint::from(n * (n + 1) / 2)
        );
        assert_eq!(
            result.registers[program.variables["count"]],
            BigUint::default()
        );
    }
}

#[test]
fn constants_above_machine_integer_limits_and_aliases() {
    let n = BigUint::from(1u8) << 128;
    let source = format!("n = {n}\nn = n\nn *= n\nhalt");
    assert_eq!(execute(&source).registers[1], BigUint::from(1u8) << 256);
    assert_eq!(
        execute("n = 144\nn = 1\nn += n\nn = 0\nhalt").registers[1],
        BigUint::default()
    );
}

#[test]
fn trace_keeps_source_and_real_land_encoding() {
    let program = compile("# hello\n\nn = 10\nhalt").unwrap();
    let frame = Machine::default().step(&program.instructions).unwrap();
    assert_eq!(frame.source_line, 3);
    assert_eq!(frame.symbols, [10, 0, 10]);
    assert_eq!(
        frame.cards,
        ["Snow-Covered Forest", "Plains", "Snow-Covered Forest"]
    );
    assert_eq!(frame.registers_before[0], "0");
    assert_eq!(frame.registers_after[0], "10");
}

#[test]
fn cyclic_execution_and_conditional_subtraction() {
    let code = [Instruction {
        symbols: [1, 1, 1],
        source_line: 1,
    }];
    let mut machine = Machine::default();
    machine.registers[1] = BigUint::from(1u8);
    assert!(!machine.step(&code).unwrap().flag_after);
    assert!(machine.step(&code).unwrap().flag_after);
    assert_eq!(machine.pc, 0);
    assert!(!machine.halted);
}

#[test]
fn bad_source_is_rejected_with_location() {
    for source in ["n = absent", "n = -1", "n = 1\nif n == 0 goto absent"] {
        assert!(compile(source).unwrap_err().starts_with("line "));
    }
}

#[test]
fn prepared_inputs_do_not_emit_runtime_assignments() {
    let source = include_str!("../examples/add-halt.mtg");
    let program = compile(source).unwrap();
    assert_eq!(
        program
            .instructions
            .iter()
            .map(|i| i.symbols)
            .collect::<Vec<_>>(),
        [[0, 1, 2], [4, 0, 0]]
    );
    assert_eq!(execute(source).registers[1], BigUint::from(5u32));
    for source in ["input a = -1\nhalt", "input a = 1\ninput a = 2\nhalt"] {
        assert!(compile(source).unwrap_err().starts_with("line "));
    }
}

#[test]
fn zero_distance_and_large_branches_relocate_correctly() {
    assert!(execute("goto next\nnext:\nhalt").halted);
    // Exceeds a single base-144 digit, checking both branch widths and labels.
    let source = format!(
        "n = 2\nresult = 0\nloop:\nif n == 0 goto done\n{}n -= 1\ngoto loop\ndone:\nhalt",
        "result += n\n".repeat(150)
    );
    let result = execute(&source);
    assert_eq!(result.registers[2], BigUint::from(450u32));
}

#[test]
fn underflow_loop_uses_immediate_branches_without_extra_scratch_writes() {
    let source = include_str!("../examples/sum-underflow.mtg");
    let code = compile(source).unwrap();
    assert_eq!(
        code.instructions
            .iter()
            .map(|i| i.symbols)
            .collect::<Vec<_>>(),
        [
            [10, 0, 3],
            [5, 2, 0],
            [0, 1, 2],
            [1, 2, 2],
            [3, 4, 1],
            [3, 1, 4],
            [4, 0, 0]
        ]
    );
    for n in 0u32..9 {
        let result = execute(&source.replace("count = 3", &format!("count = {n}")));
        assert_eq!(result.registers[1], BigUint::from(n * (n + 1) / 2));
        assert!(result.flag);
    }
    let source = format!(
        "input n = 0\nloop:\n{}goto loop\nhalt",
        "n += n\n".repeat(10)
    );
    assert_eq!(
        compile(&source).unwrap().instructions[10].symbols,
        [3, 1, 11]
    );
}
