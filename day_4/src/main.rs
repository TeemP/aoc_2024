use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_file(path: &str) -> Vec<Vec<char>> {
    let file = File::open(path).expect("Unable to read open the path");
    let reader = BufReader::new(file);
    reader.lines()
        .map_while(Result::ok)
        .map(|line| line.chars().collect())
        .collect()
}

fn find_horizontal(needle: &str, input: &[Vec<char>]) -> usize {
    let reverse_needle = &needle.chars().rev().collect::<String>();
    input.iter()
        .map(|line| 
            line.windows(needle.len())
            .filter(|chunk| {
                let hay_string = chunk.iter().collect::<String>();
                (hay_string == needle) | (&hay_string == reverse_needle)
            }).count()
        ).sum()
}

fn find_vertical(needle: &str, input: &[Vec<char>]) -> usize {
    let needle_len = needle.len();
    let reverse_needle = &needle.chars().rev().collect::<String>();
    let y_len = input.len() - needle_len;
    let x_len = input[0].len();
    let mut count = 0;
    for x in 0..x_len {
        for y in 0..y_len+1 {
            let hay_string = &input[y..(y+needle_len)].iter()
                .map(|row| row[x])
                .collect::<String>();
            if (hay_string == needle) | (hay_string == reverse_needle) {
                count += 1;
            }
        }
    }
    count
}

fn find_diagonal_down(needle: &str, input: &[Vec<char>]) -> usize {
    let needle_len = needle.len();
    let reverse_needle = &needle.chars().rev().collect::<String>();
    let y_len = input.len() - needle_len;
    let x_len = input[0].len() - needle_len;
    let mut count = 0;
    for x in 0..x_len+1 {
        for y in 0..y_len+1 {
            let hay_string = &input[y..(y+needle_len)].iter()
                .enumerate()
                .map(|(i,row)| row[x+i])
                .collect::<String>();
            if (hay_string == needle) | (hay_string == reverse_needle) {
                count += 1;
            }
        }
    }
    count
}

fn find_diagonal_up(needle: &str, input: &[Vec<char>]) -> usize {
    let needle_len = needle.len();
    let reverse_needle = &needle.chars().rev().collect::<String>();
    let y_len = input.len();
    let x_len = input[0].len();
    let mut count = 0;
    for x in needle_len-1..x_len {
        for y in needle_len-1..y_len {
            let hay_string = &input[(y+1-needle_len)..y+1].iter()
                .enumerate()
                .map(|(i,row)| row[x-i])
                .collect::<String>();
            if (hay_string == needle) | (hay_string == reverse_needle) {
                count += 1;
            }
        }
    }
    count
}

fn find_x_sam(input: &[Vec<char>]) -> usize {
    let needle = "MAS";
    let needle_len = needle.len();
    let reverse_needle = &needle.chars().rev().collect::<String>();
    let y_len = input.len() - needle_len;
    let x_len = input[0].len() - needle_len;
    let mut count = 0;
    for x in 0..x_len + 1{
        for y in 0..y_len + 1 {
            let hay_string_1 = &input[y..(y+needle_len)].iter()
                .enumerate()
                .map(|(i,row)| row[x+i])
                .collect::<String>();
            let hay_string_2 = &input[y..(y+needle_len)].iter()
                .enumerate()
                .map(|(i,row)| row[x+needle_len-i-1])
                .collect::<String>();
            let hay_string_1_found = (hay_string_1 == needle) | (hay_string_1 == reverse_needle);
            let hay_string_2_found = (hay_string_2 == needle) | (hay_string_2 == reverse_needle);
            if hay_string_1_found & hay_string_2_found {
                count += 1;
            }
        }
    }
    count
}

fn find_all_words_from_input(needle: &str, input: &[Vec<char>]) -> usize {
    let mut sum = find_horizontal(needle, input);
    sum += find_vertical(needle, input);
    sum += find_diagonal_down(needle, input);
    sum += find_diagonal_up(needle, input);
    sum
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = read_file(&args[1]);
    let needle = "XMAS";
    println!("word \"{}\" was found {} times",
        &needle,
        find_all_words_from_input(needle, &input)
    );
    println!("X-MAS was found {} times",find_x_sam(&input));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_horizontal() {
        let input = read_file("test_data.txt");
        let needle = "XMAS";
        let result = find_horizontal(needle, &input);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_find_vertical() {
        let input = read_file("test_data.txt");
        let needle = "XMAS";
        let result = find_vertical(needle, &input);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_find_diagonal_down() {
        let input = read_file("test_data.txt");
        let needle = "XMAS";
        let result = find_diagonal_down(needle, &input);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_find_diagonal_up() {
        let input = read_file("test_data.txt");
        let needle = "XMAS";
        let result = find_diagonal_up(needle, &input);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_find_x_sam() {
        let input = read_file("test_data.txt");
        let result = find_x_sam(&input);
        assert_eq!(result, 9);
    }
}
