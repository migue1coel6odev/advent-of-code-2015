use std::fmt::Display;

use regex::Regex;

const SIZE: usize = 1000;

pub struct LightsManagerV2 {
    lights: [[u8; SIZE]; SIZE],
    total_brightness: usize
}

impl LightsManagerV2 {
    pub fn new() -> Self {
        let lights = [[0; SIZE]; SIZE];
        Self { lights, total_brightness: 0 }
    }

    pub fn run_instruction(&mut self, instruction: &String) {
        let re = Regex::new(r"(?<action>(turn (on|off)|toggle)) (?<x>[0-9]*),(?<y>[0-9]*) through (?<x2>[0-9]*),(?<y2>[0-9]*)").unwrap();

        let result = re.captures(instruction).unwrap();

        let from_x: usize = result["x"].parse().unwrap();
        let from_y: usize = result["y"].parse().unwrap();
        let to_x: usize = result["x2"].parse::<usize>().unwrap() + 1;
        let to_y: usize = result["y2"].parse::<usize>().unwrap() + 1;

        match &result["action"] {
            "turn on" => {
                for x in from_x..to_x {
                    for y in from_y..to_y {
                        self.lights[x][y] += 1;
                        self.total_brightness += 1;
                    }
                }
            }
            "turn off" => {
                for x in from_x..to_x {
                    for y in from_y..to_y {
                        if self.lights[x][y] > 0 {
                            self.lights[x][y] -= 1;
                            self.total_brightness -= 1;
                        }
                    }
                }
            }
            "toggle" => {
                for x in from_x..to_x {
                    for y in from_y..to_y {
                        self.lights[x][y] += 2;
                        self.total_brightness += 2;
                    }
                }
            }
            _ => {}
        }
    }

    pub fn get_total_brightness(&self) -> usize {
        self.total_brightness
    }
}

impl Display for LightsManagerV2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut display = String::new();

        display.push('|');
        display.push_str(&["-"; SIZE].join(""));
        display.push('|');
        display.push('\n');
        for line in self.lights {
            display.push('|');
            display.push_str(&line.map(|c| c.to_string()).join(""));
            display.push('|');
            display.push('\n');
        }
        display.push('|');
        display.push_str(&["-"; SIZE].join(""));
        display.push('|');

        write!(f, "{}", display)
    }
}
