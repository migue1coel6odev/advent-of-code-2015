use std::fs;

fn main() {
    let warehouse = read_file();

    println!(
        "Total square feet of wrapping paper needed is: {}",
        warehouse.calculate_total_paper_needed()
    );
    println!(
        "Total feet of ribbon needed is: {}",
        warehouse.calculate_total_ribbon_needed()
    );
}

fn read_file() -> ElfWarehouse {
    let present_list = fs::read("./present_list.txt").unwrap();
    let list_to_string = String::from_utf8(present_list).unwrap();

    let lines = list_to_string.split("\n");

    let mut warehouse = ElfWarehouse::new();

    for line in lines {
        let splitted: Vec<&str> = line.split("x").collect();

        let present = Present::new(splitted).unwrap();

        warehouse.add_present(present);
    }

    warehouse
}

#[derive(Debug)]
struct Present {
    l: usize,
    h: usize,
    w: usize,
}

impl Present {
    fn new(measures: Vec<&str>) -> Option<Self> {
        if measures.len() != 3 {
            return None;
        }

        let mut result: Vec<usize> = measures
            .iter()
            .map(|value| value.parse::<usize>().unwrap())
            .collect();
        result.sort();

        let l: usize = result.get(0).unwrap().clone();
        let h: usize = result.get(1).unwrap().clone();
        let w: usize = result.get(2).unwrap().clone();

        Some(Present { l, h, w })
    }

    fn calculate_needed_paper(&self) -> usize {
        2 * self.l * self.w + 2 * self.w * self.h + 2 * self.h * self.l + self.l * self.h
    }

    fn calculate_needed_ribbon(&self) -> usize {
        2 * self.l + 2 * self.h + self.h * self.l * self.w
    }
}

struct ElfWarehouse {
    presents: Vec<Present>,
}

impl ElfWarehouse {
    fn new() -> Self {
        let presents: Vec<Present> = Vec::new();

        ElfWarehouse { presents }
    }

    fn add_present(&mut self, present: Present) {
        let _ = &self.presents.push(present);
    }

    fn calculate_total_paper_needed(&self) -> usize {
        self.presents
            .iter()
            .map(|present| present.calculate_needed_paper())
            .sum()
    }

    fn calculate_total_ribbon_needed(&self) -> usize {
        self.presents
            .iter()
            .map(|present| present.calculate_needed_ribbon())
            .sum()
    }
}
