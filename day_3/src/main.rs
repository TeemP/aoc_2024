use std::env;
use std::fs::File;
use std::io::Read;

enum MulState {
    Begin,
    M,
    U,
    L,
    LeftBracket,
    FirstNumeric(u32),
    Sep,
    SecondNumeric(u32)
}

enum EnabledState {
    Begin,
    D,
    O,
    N,
    Apostrophe,
    T,
    LeftBracket
}

fn parse_memory_string(string: &str, enable_command: bool) -> u32 {
    let mut mul_state = MulState::Begin;
    let mut enabled_state = EnabledState::Begin;
    let mut sum = 0;
    let mut first_multiplier= 0;
    let mut enabled = true;
    for parsed_character in string.chars(){
        match mul_state {
            MulState::Begin => {
                if parsed_character == 'm' {
                    mul_state = MulState::M;
                } else {
                    mul_state = MulState::Begin;
                }
            },
            MulState::M => {
                if parsed_character == 'u' {
                    mul_state = MulState::U;
                } else {
                    mul_state = MulState::Begin;
                }
            },
            MulState::U =>  {
                if parsed_character == 'l' {
                    mul_state = MulState::L;
                } else {
                    mul_state = MulState::Begin;
                }
            },
            MulState::L =>  {
                if parsed_character == '(' {
                    mul_state = MulState::LeftBracket;
                } else {
                    mul_state = MulState::Begin;
                }
            },
            MulState::LeftBracket => {
                if parsed_character.is_numeric() {
                    mul_state = MulState::FirstNumeric(parsed_character.to_digit(10).unwrap());
                } else {
                    mul_state = MulState::Begin;
                }
            },
            MulState::FirstNumeric(numeric_value) => {
                if parsed_character.is_numeric() {
                    mul_state = MulState::FirstNumeric(
                        numeric_value * 10 + 
                        parsed_character.to_digit(10)
                        .unwrap()
                    );
                } else if  parsed_character == ',' {
                    first_multiplier = numeric_value;
                    mul_state = MulState::Sep;
                } else {
                    mul_state = MulState::Begin;
                }
            },
            MulState::Sep =>  if parsed_character.is_numeric() {
                mul_state = MulState::SecondNumeric(parsed_character.to_digit(10).unwrap());
            },
            MulState::SecondNumeric(numeric_value) =>  {
                if parsed_character.is_numeric() {
                    mul_state = MulState::SecondNumeric(
                        numeric_value * 10 + 
                        parsed_character.to_digit(10)
                        .unwrap()
                    );
                } else if  parsed_character == ')' {
                    if enabled {
                        sum += first_multiplier * numeric_value;
                    }
                    mul_state = MulState::Begin;
                } else {
                    mul_state = MulState::Begin;
                }
            }
        }
        if enable_command {
            match enabled_state {
                EnabledState::Begin => {
                    if parsed_character == 'd' {
                        enabled_state = EnabledState::D;
                    } else {
                        enabled_state = EnabledState::Begin;
                    }
                },
                EnabledState::D => {
                    if parsed_character == 'o' {
                        if enabled {
                            enabled_state = EnabledState::O;
                        }
                        else {
                            enabled_state = EnabledState::T
                        }
                    } else {
                        enabled_state = EnabledState::Begin;
                    }
                },
                EnabledState::O => {
                    if parsed_character == 'n' {
                        enabled_state = EnabledState::N;
                    } else {
                        enabled_state = EnabledState::Begin;
                    }
                },
                EnabledState::N => {
                    if parsed_character == '\'' {
                        enabled_state = EnabledState::Apostrophe;
                    } else {
                        enabled_state = EnabledState::Begin;
                    }
                },
                EnabledState::Apostrophe => {
                    if parsed_character == 't' {
                        enabled_state = EnabledState::T;
                    } else {
                        enabled_state = EnabledState::Begin;
                    }
                },
                EnabledState::T => {
                    if parsed_character == '(' {
                        enabled_state = EnabledState::LeftBracket;
                    } else {
                        enabled_state = EnabledState::Begin;
                    }
                },
                EnabledState::LeftBracket => {
                    if parsed_character == ')' {
                        enabled_state = EnabledState::Begin;
                        enabled = !enabled;
                    } else {
                        enabled_state = EnabledState::Begin;
                    }
                },
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
        parse_memory_string(&memory_string, false)
    );
    println!("Sum of all multiplications with enabled-command: {}", 
        parse_memory_string(&memory_string, true)
    )
}
