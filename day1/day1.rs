use std::fs;

fn main() {
    let file_path = "input";

    let contents = fs::read_to_string(file_path)
        .expect("couldn't read file");

    let mut sum = 0;

    for line in contents.lines() {
        let mut numstring = "".to_owned();
        for digit in line.chars() {
            if digit.is_numeric() {
                numstring.push(digit);
            }
        }
        let mut first_and_last = numstring.chars().next().unwrap().to_owned().to_string();
        first_and_last.push(numstring.chars().last().unwrap());
        sum += first_and_last.parse::<i32>().unwrap();
    }
    println!("{}",sum);
}
