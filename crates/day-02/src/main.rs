#[derive(Debug, PartialEq)]
struct Report {
    levels: Vec<u8>,
}

fn main() {
    let reports = parse(include_str!("../input/real"));
    for report in &reports {
        let safe = report.is_safe();
        println!("{report:?} is safe: {safe}");
    }
    let solution_part1 = solve_part1(&reports);
    println!("Part 1: {solution_part1}");
}

fn parse(input: &str) -> Vec<Report> {
    input
        .lines()
        .filter_map(|line| {
            let levels: Vec<u8> = line
                .split_whitespace()
                .map(|level| level.parse().unwrap())
                .collect();
            if levels.is_empty() {
                return None;
            }
            Some(Report { levels })
        })
        .collect()
}

fn solve_part1(reports: &[Report]) -> usize {
    reports.iter().filter(|report| report.is_safe()).count()
}

fn solve_part2() {
    todo!()
}

impl Report {
    fn is_safe(&self) -> bool {
        let a = self.levels.windows(2).all(|window| {
            if window[1] < window[0] {
                return false;
            }
            (1..=3).contains(&(window[1] - window[0]))
        });
        let b = self.levels.windows(2).all(|window| {
            if window[0] < window[1] {
                return false;
            }
            (1..=3).contains(&(window[0] - window[1]))
        });
        a || b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let reports = parse(include_str!("../input/test"));
        assert_eq!(
            reports,
            vec![
                Report {
                    levels: vec![7, 6, 4, 2, 1]
                },
                Report {
                    levels: vec![1, 2, 7, 8, 9]
                },
                Report {
                    levels: vec![9, 7, 6, 2, 1]
                },
                Report {
                    levels: vec![1, 3, 2, 4, 5]
                },
                Report {
                    levels: vec![8, 6, 4, 4, 1]
                },
                Report {
                    levels: vec![1, 3, 6, 7, 9]
                },
            ],
        );
    }

    #[test]
    fn test_report_safe_1() {
        let report = Report {
            levels: vec![1, 4, 5, 6, 8],
        };
        assert!(report.is_safe());
    }

    #[test]
    fn test_report_safe_2() {
        let report = Report {
            levels: vec![8, 6, 5, 4, 1],
        };
        assert!(report.is_safe());
    }

    #[test]
    fn test_report_unsafe_1() {
        let report = Report {
            levels: vec![1, 4, 8, 9, 10],
        };
        assert!(!report.is_safe());
    }

    #[test]
    fn test_report_unsafe_2() {
        let report = Report {
            levels: vec![1, 4, 3, 5, 6],
        };
        assert!(!report.is_safe());
    }

    #[test]
    fn test_solve_part1() {
        let reports = parse(include_str!("../input/test"));
        assert_eq!(solve_part1(&reports), 2);
    }

    #[test]
    fn test_solve_part2() {}
}
