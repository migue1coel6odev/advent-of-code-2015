use std::{collections::HashMap, fs};

fn main() {
    let directions = read_file();

    part_1(&directions);
    part_1_refactor(&directions);
    part_2(&directions);
}

fn part_2(directions: &String) {
    let mut houses_map: HashMap<String, usize> = HashMap::new();

    houses_map.insert(format!("{}|{}", 0, 0), 0);

    let mut santa_x = 0;
    let mut santa_y = 0;
    directions
        .chars()
        .skip(0)
        .step_by(2)
        .for_each(|symbol| part_2_util(&mut houses_map, &mut santa_x, &mut santa_y, symbol));

    let mut robo_x = 0;
    let mut robo_y = 0;
    directions
        .chars()
        .skip(1)
        .step_by(2)
        .for_each(|symbol| part_2_util(&mut houses_map, &mut robo_x, &mut robo_y, symbol));

    // dbg!(houses_map);

    println!(
        "| PART 2  | Houses with at least one present equals to: {}",
        houses_map.len()
    )
}

fn part_2_util(
    houses_map: &mut HashMap<String, usize>,
    x: &mut isize,
    y: &mut isize,
    symbol: char,
) {
    match symbol {
        '^' => {
            *y += 1;
            *houses_map.entry(format!("{}|{}", x, y)).or_insert(0) += 1;
        }
        'v' => {
            *y -= 1;
            *houses_map.entry(format!("{}|{}", x, y)).or_insert(0) += 1;
        }
        '>' => {
            *x += 1;
            *houses_map.entry(format!("{}|{}", x, y)).or_insert(0) += 1;
        }
        '<' => {
            *x -= 1;
            *houses_map.entry(format!("{}|{}", x, y)).or_insert(0) += 1;
        }
        _ => {}
    };
}

fn part_1_refactor(directions: &String) {
    let mut houses_map: HashMap<String, usize> = HashMap::from([("0|0".to_string(), 0)]);

    let mut x = 0;
    let mut y = 0;

    directions
        .chars()
        .for_each(|symbol| part_2_util(&mut houses_map, &mut x, &mut y, symbol));

    println!(
        "| PART 1* | Houses with at least one present equals to: {}",
        houses_map.len()
    )
}

fn part_1(directions: &String) {
    let mut houses_map: HashMap<String, usize> = HashMap::new();

    let mut x = 0;
    let mut y = 0;
    houses_map.insert(format!("{}|{}", x, y), 0);

    directions.chars().for_each(|direction| match direction {
        '^' => {
            y += 1;
            *houses_map.entry(format!("{}|{}", x, y)).or_insert(0) += 1;
        }
        'v' => {
            y -= 1;
            *houses_map.entry(format!("{}|{}", x, y)).or_insert(0) += 1;
        }
        '>' => {
            x += 1;
            *houses_map.entry(format!("{}|{}", x, y)).or_insert(0) += 1;
        }
        '<' => {
            x -= 1;
            *houses_map.entry(format!("{}|{}", x, y)).or_insert(0) += 1;
        }
        _ => {}
    });

    println!(
        "| PART 1  | Houses with at least one present equals to: {}",
        houses_map.len()
    )
}

fn read_file() -> String {
    String::from_utf8(fs::read("directions.txt").unwrap()).unwrap()
}
