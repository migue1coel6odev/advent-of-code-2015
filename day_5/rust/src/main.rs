use std::fs;

fn main() {
    
    let strings = read_file("strings.txt");

    let result: Vec<&String>  = strings.iter().filter(|word| check_nice_string(word)).collect();
    println!("| PART 1 | The number of nice strings is: {}", result.len());
    
    let result: Vec<&String>  = strings.iter().filter(|word| check_nice_string_v2(word)).collect();
    println!("| PART 2 | The number of nice strings is: {}", result.len());

}

fn read_file(path: &str) -> Vec<String> {

    let strings = fs::read(path).unwrap();

    String::from_utf8(strings).unwrap().split("\n").map(|s| s.to_string()).collect()
}

fn check_nice_string(str: &String) -> bool {

    let mut previous_char: Option<char> = None;

    let mut count_vowels = 0;
    let mut check_repeated_char = false;
    let mut is_naughty = false;

    str.chars().for_each(|c| {

        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                count_vowels += 1;    
            },
            _ => {}
        }

        if let Some(prev_char) = previous_char {
            
            if check_repeated_char == false && prev_char == c {
                check_repeated_char = true;
            }

            match c {
                'b' => {
                    if prev_char == 'a' {
                        is_naughty = true;
                    }
                },
                'd' => {
                    if prev_char == 'c' {
                        is_naughty = true;
                    }
                },
                'q' => {
                    if prev_char == 'p' {
                        is_naughty = true;
                    }
                },
                'y' => {
                    if prev_char == 'x' {
                        is_naughty = true;
                    }
                }, 
                _ => {},
            }

        }

        previous_char = Some(c);
    });

    count_vowels >= 3 && check_repeated_char && !is_naughty

}


fn check_nice_string_v2(str: &String) -> bool {

    let mut last_char: Option<char> = None;
    let mut second_to_last_char: Option<char> = None;

    let mut check_same_with_letter_between = false;

    str.chars().for_each(|current_char| {

        if let Some(second_to_last_char) = second_to_last_char {
            if second_to_last_char == current_char {
                check_same_with_letter_between = true;
            }
        }

        if let Some(prev_char) = last_char {
            second_to_last_char = Some(prev_char);
        }
        last_char = Some(current_char);
    });

    let mut index = 0;
    let mut found_repeat_duo = false;
    while index < str.len() - 1 {
        let check_pattern = str.get(index..index+2).unwrap();
        let rest = str.get(index + 2..).unwrap();

        if rest.contains(&check_pattern) {
            found_repeat_duo = true;
            break;
        }

        index += 1;
    }

    check_same_with_letter_between && found_repeat_duo

}

#[cfg(test)]
mod tests {
    use crate::{read_file, check_nice_string, check_nice_string_v2};


    #[test]
    fn check_test_file() {
        let test_file = read_file("test.txt");

        let result: Vec<&String>  = test_file.iter().filter(|word| check_nice_string(word)).collect();

        assert_eq!(result.len(), 2);
    }

    #[test]
    fn check_test_file_part_2() {
        let test_file = read_file("test_2.txt");

        let result: Vec<&String>  = test_file.iter().filter(|word| check_nice_string_v2(word)).collect();

        assert_eq!(result.len(), 2);
    }
}