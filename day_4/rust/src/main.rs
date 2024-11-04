use md5::Digest;

fn main() {
    
    let puzzle_input = "ckczppom";

    let (number, digest) = find_solution(&puzzle_input);
    println!("| PART 1 | The number is: {} which means this is the key: {}{}, that generates this md5 hash: {:x} ", number, puzzle_input, number, digest);
    
    let (number, digest) = find_solution_part2(&puzzle_input);
    println!("| PART 2 | The number is: {} which means this is the key: {}{}, that generates this md5 hash: {:x} ", number, puzzle_input, number, digest);
    
}

fn find_solution(puzzle_input: &str) -> (i32, Digest){
    let mut number = 1;
    loop {
        let digest = md5::compute(format!("{}{}", puzzle_input,number));
        if format!("{:x}", &digest).starts_with("00000") {
            return (number, digest);
        }
        number += 1;
    }
}

fn find_solution_part2(puzzle_input: &str) -> (i32, Digest){
    let mut number = 1;
    loop {
        let digest = md5::compute(format!("{}{}", puzzle_input,number));
        if format!("{:x}", &digest).starts_with("000000") {
            return (number, digest);
        }
        number += 1;
    }
}