mod light_manager;
mod light_manager_v2;

use std::fs;

use light_manager::LightsManager;
use light_manager_v2::LightsManagerV2;

fn main() {
    let instructions = read_file();

    let mut light_manager = LightsManager::new();
    instructions.iter().for_each(|instruction| light_manager.run_instruction(instruction));
    println!("| PART 1 | Currently there's {} lights on!", light_manager.get_lights_on());

    let mut light_manager_v2 = LightsManagerV2::new();
    instructions.iter().for_each(|instruction| light_manager_v2.run_instruction(instruction));
    println!("| PART 1 | Currently the total brightness is: {}", light_manager_v2.get_total_brightness());
    
} 

fn read_file() -> Vec<String> {
    String::from_utf8(fs::read("light_instructions.txt").unwrap())
        .unwrap()
        .split("\n")
        .map(|s| s.to_string())
        .collect()
}

#[cfg(test)]
mod test {
    use crate::read_file;

    #[test]
    fn test_instructions() {
        let file = read_file();
    }
}
