use std::env;
use std::fs::File;
use std::io::Read;


enum State {
    Begin,
    M,
    U,
    L,
    LeftBracket,
    FirstNumeric(u32),
    Sep,
    SecondNumeric(u32)
}

fn parse_memory_string(string: &str) -> u32 {
    let mut state = State::Begin;
    let mut sum = 0;
    let mut first_multiplier= 0;
    for parsed_character in string.chars(){
        match state {
            State::Begin => {
                if parsed_character == 'm' {
                    state = State::M;
                } else {
                    state = State::Begin;
                }
            },
            State::M => {
                if parsed_character == 'u' {
                    state = State::U;
                } else {
                    state = State::Begin;
                }
            },
            State::U =>  {
                if parsed_character == 'l' {
                    state = State::L;
                } else {
                    state = State::Begin;
                }
            },
            State::L =>  {
                if parsed_character == '(' {
                    state = State::LeftBracket;
                } else {
                    state = State::Begin;
                }
            },
            State::LeftBracket => {
                if parsed_character.is_numeric() {
                    state = State::FirstNumeric(parsed_character.to_digit(10).unwrap());
                } else {
                    state = State::Begin;
                }
            },
            State::FirstNumeric(numeric_value) => {
                if parsed_character.is_numeric() {
                    state = State::FirstNumeric(
                        numeric_value * 10 + 
                        parsed_character.to_digit(10)
                        .unwrap()
                    );
                } else if  parsed_character == ',' {
                    first_multiplier = numeric_value;
                    state = State::Sep;
                } else {
                    state = State::Begin;
                }
            },
            State::Sep =>  if parsed_character.is_numeric() {
                state = State::SecondNumeric(parsed_character.to_digit(10).unwrap());
            },
            State::SecondNumeric(numeric_value) =>  {
                if parsed_character.is_numeric() {
                    state = State::SecondNumeric(
                        numeric_value * 10 + 
                        parsed_character.to_digit(10)
                        .unwrap()
                    );
                } else if  parsed_character == ')' {
                    sum += first_multiplier * numeric_value;
                    state = State::Begin;
                } else {
                    state = State::Begin;
                }
            }
        }
    }
    sum
}

fn read_file(path: &str) -> String {
    let mut file = File::open(path).expect("Unable to read open the path");
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).expect("Failed to read non UTF8 byte sequence from the file");
    buffer
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let memory_string = read_file(&args[1]);
    println!("Sum of all multiplications: {}", 
        parse_memory_string(&memory_string)
    )
}
