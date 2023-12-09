use std::fs;
// use std::str::Chars;


fn find_first_digit(chars: impl Iterator<Item=char>) -> Option<u32>{
    for chr in chars{
        if chr.is_ascii_digit() {
            return Some(chr.to_digit(10).unwrap());
        }
    }
    None
}


fn decode_calibrations_part1(filename: &str) -> u32{
    let content = fs::read_to_string(filename).unwrap();

    let mut total_calibration_value: u32 = 0;

    // Look for first digit
    for line in content.split("\n"){
        // println!("LINE {}", line);

        let possible_left_match = find_first_digit(line.chars());
        let left_match = possible_left_match.unwrap();
        total_calibration_value += left_match * 10;


        let possible_right_match: Option<u32> = find_first_digit(line.chars().rev());
        let right_match = possible_right_match.unwrap();
        total_calibration_value += right_match;

        // println!("F {} S {}", left_match, right_match);

    }  // for line in file

    total_calibration_value
}


fn decode_calibrations_part2(filename: &str) -> u32{
    let content = fs::read_to_string(filename).unwrap();

    const NUMBERS: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut total_calibration_value: u32 = 0;

    // Look for first digit
    for line in content.split("\n"){
        // println!("LINE {}", line);

        let mut possible_left_match: Option<u32> = None;
        'char_in_line: for (index, chr) in line.chars().enumerate(){

            if chr.is_ascii_digit() {
                possible_left_match = Some(chr.to_digit(10).unwrap());
                break;
            }

            for (number_index, number) in NUMBERS.iter().enumerate(){
                if line[index..].starts_with(*number){
                    possible_left_match = Some(number_index as u32 + 1);
                    break 'char_in_line;
                }
            }
        }
        let left_match = possible_left_match.unwrap();
        total_calibration_value += left_match * 10;

        let mut possible_right_match: Option<u32> = None;
        'char_in_line: for index in (0..line.len()).rev(){
            let chr = line.as_bytes()[index] as char;

            if chr.is_ascii_digit(){
                possible_right_match = Some(chr.to_digit(10).unwrap());
                break;
            }

            for (number_index, number) in NUMBERS.iter().enumerate(){
                if line[index..].starts_with(*number){
                    possible_right_match = Some(number_index as u32 + 1);
                    break 'char_in_line;
                }
            }
        }
        let right_match = possible_right_match.unwrap();
        total_calibration_value += right_match;

        // println!("F {} S {}", left_match, right_match);

    }  // for line in file

    total_calibration_value
}


fn main() {
    println!("Part 1");
    let sample_result = decode_calibrations_part1("sample.txt");
    println!("Sample result: {}", sample_result);
    let input_result = decode_calibrations_part1("input.txt");
    println!("Input result: {}", input_result);


    println!("Part 2");
    let sample_result = decode_calibrations_part2("sample2.txt");
    println!("Sample result: {}", sample_result);
    let input_result = decode_calibrations_part2("input2.txt");
    println!("Sample result: {}", input_result);
}

