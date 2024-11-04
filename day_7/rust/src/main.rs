use std::fs;

use cpu::Cpu;

mod cpu;

fn main() {
    let mut instructions = read_file();

    let mut cpu = Cpu::new();

    loop {
        if instructions.len() == 0 {
            break;
        }
        instructions = test_loop(&mut instructions, &mut cpu);
    }
    
    println!("The power for the wire a is: {}", cpu.get_wire_power("a"));
}

fn test_loop(instructions: &mut Vec<String>, cpu: &mut Cpu) -> Vec<String>{
    let mut temp_instructions :  Vec<String> = vec![];

    instructions.iter().for_each(|instruction| {
        if cpu.run_instruction(instruction.clone()).is_none() {
            temp_instructions.push(instruction.clone());
        };
    });

    temp_instructions
}

fn read_file() -> Vec<String> {
    String::from_utf8(fs::read("instructions.txt").unwrap())
        .unwrap()
        .split("\n")
        .map(|s| s.to_string())
        .collect()
}
