use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};



fn read_file(path: &str) -> Vec<Vec<i32>> {
    let file = File::open(path).expect("Unable to read open the path");
    let reader = BufReader::new(file);
    reader.lines()
        .map_while(Result::ok)
        .map(|s|{
             s.split(" ")
            .filter_map(|p| p.parse().ok())
            .collect::<Vec<i32>>()
        }).collect()
}



fn check_safety(report: &[i32]) -> bool {
    let direction = (report[0] - report[1]).is_negative();
    report.windows(2)
        .all(|pair|{
            let step = pair[0] - pair[1];
            let right_direction = step.is_negative() == direction;
            let step = step.abs();
            (0 < step) & (step <4) & right_direction
        })
}



fn count_safe_reports(reports: &[Vec<i32>]) -> usize {
    reports.iter()
    .skip_while(|report| {
        !check_safety(report)
    }).count()
}



fn main() {
    let args: Vec<String> = env::args().collect();
    let reports = read_file(&args[1]);
    let safe_count = count_safe_reports(&reports);
    println!("Safe report count: {}", safe_count);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_safety_direction() {
        let input = vec![1,2,3];
        let result = check_safety(&input);
        assert_eq!(result, true);
    }

    #[test]
    fn test_check_safety_neg_direction() {
        let input = vec![-1,-2,-3];
        let result = check_safety(&input);
        assert_eq!(result, true);
    }

    #[test]
    fn test_check_safety_wrong_direction() {
        let input = vec![1,2,1];
        let result = check_safety(&input);
        assert_eq!(result, false);
    }

    #[test]
    fn test_check_safety_wrong_neg_direction() {
        let input = vec![-1,-2,-1];
        let result = check_safety(&input);
        assert_eq!(result, false);
    }

    #[test]
    fn test_check_safety_no_step() {
        let input = vec![-1,-2,-2,-3];
        let result = check_safety(&input);
        assert_eq!(result, false);
    }
}
