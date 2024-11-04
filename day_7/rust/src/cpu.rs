use std::collections::HashMap;

use regex::Regex;

#[derive(Debug)]
pub struct Cpu {
    wires: HashMap<String, u16>,
    reg: Regex,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            wires: HashMap::new(),
            reg: Regex::new(r"(NOT (?<not>[a-z]*)|((?<left>[a-z0-9]*) (?<operator>AND|OR|LSHIFT|RSHIFT) (?<right>[0-9a-z]*))|(?<number>[0-9a-z]*)) -> (?<out>[a-z]*)").unwrap()
        }
    }

    pub fn run_instruction(&mut self, instruction: String) -> Option<()> {
        let ins = instruction.as_str();

        let result = self.reg.captures(ins).unwrap();

        let not = result.name("not");
        let left = result.name("left");
        let number = result.name("number");
        let out = result.name("out").unwrap().as_str();

        if let Some(not) = not {
            let not = not.as_str();

            let not_value = match self.wires.get(not) {
                Some(x) => x,
                None => return None,
            };

            self.wires.insert(String::from(out), !*not_value);

            return Some(());
        }

        if let Some(left) = left {
            let left = left.as_str();
            let operator = result.name("operator").unwrap().as_str();
            let right = result.name("right").unwrap().as_str();

            let left = match self.wires.get(left) {
                Some(x) => *x,
                None => match left.parse::<u16>() {
                    Ok(x) => x,
                    _ => return None,
                },
            };

            let right = match self.wires.get(right) {
                Some(x) => *x,
                None => match right.parse::<u16>() {
                    Ok(x) => x,
                    _ => return None,
                },
            };

            match operator {
                "AND" => {
                    self.wires.insert(String::from(out), left & right);
                }
                "OR" => {
                    self.wires.insert(String::from(out), left | right);
                }
                "LSHIFT" => {
                    self.wires.insert(String::from(out), left << right);
                }
                "RSHIFT" => {
                    self.wires.insert(String::from(out), left >> right);
                }
                _ => (),
            }

            return Some(());
        }

        if let Some(number) = number {
            let number = number.as_str();
            let number = match self.wires.get(number) {
                Some(x) => *x,
                None => match number.parse::<u16>() {
                    Ok(x) => x,
                    _ => return None,
                },
            };
            self.wires.insert(String::from(out), number);
            return Some(());
        }

        None
    }

    pub fn get_wire_power(&self, wire: &str) -> u16 {
        self.wires.get(&String::from(wire)).unwrap().clone()
    }
}
