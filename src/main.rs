use ::regex::Regex;
use std::fs;
use std::io;

fn homework_1() {
    let org_arr = [1, 2, 3, 5, 6, 10, 8, 11];
    let sub_arr = [6, 8, 10];
    let mut result = String::new();

    for x in org_arr.iter() {
        if sub_arr.contains(x) {
            result.push_str(&x.to_string());
        }
    }

    let sub_arr_to_string = sub_arr.iter().map(|&x| x.to_string()).collect::<String>();

    if result == sub_arr_to_string {
        println!("true");
        return;
    }

    println!("false");
}

fn homework_2() {
    let mut input = String::new();
    println!("Enter words: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.pop();

    let contents = fs::read_to_string("./src/paragraph.txt").expect("Failed to read file");

    let re = Regex::new(&format!("(?i){}", regex::escape(&input))).unwrap();

    // count all word match with regex

    let count = re.find_iter(&contents).count();

    println!("Times repeat of \"{}\": {}", input, count);
}

fn main() {
    homework_1();
    homework_2();
}
