fn main() {
    let (lhs, rhs) = parse(include_str!("../input/real"));
    let solution_part2 = solve_part2(&lhs, &rhs);
    let solution_part1 = solve_part1(lhs, rhs);
    println!("Part 1: {solution_part1}");
    println!("Part 2: {solution_part2}");
}

fn parse(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut lhs_vec = Vec::new();
    let mut rhs_vec = Vec::new();
    for line in input.lines() {
        let mut iter = line.split_whitespace();
        let lhs = iter.next().unwrap().parse().unwrap();
        let rhs = iter.next().unwrap().parse().unwrap();
        lhs_vec.push(lhs);
        rhs_vec.push(rhs);
    }
    (lhs_vec, rhs_vec)
}

fn solve_part1(mut lhs: Vec<i32>, mut rhs: Vec<i32>) -> i32 {
    lhs.sort();
    rhs.sort();
    lhs.into_iter()
        .zip(rhs)
        .fold(0, |acc, (lhs, rhs)| acc + (lhs - rhs).abs())
}

fn solve_part2(lhs: &[i32], rhs: &[i32]) -> i32 {
    lhs.iter().fold(0, |acc, lhs| {
        let count = rhs.iter().filter(|&rhs| lhs == rhs).count() as i32;
        let similarity = lhs * count;
        acc + similarity
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let (lhs, rhs) = parse(include_str!("../input/test"));
        assert_eq!(lhs, vec![3, 4, 2, 1, 3, 3]);
        assert_eq!(rhs, vec![4, 3, 5, 3, 9, 3]);
    }

    #[test]
    fn test_solve_part1() {
        let (lhs, rhs) = parse(include_str!("../input/test"));
        let solution = solve_part1(lhs, rhs);
        assert_eq!(solution, 11);
    }

    #[test]
    fn test_solve_part2() {
        let (lhs, rhs) = parse(include_str!("../input/test"));
        let solution = solve_part2(&lhs, &rhs);
        assert_eq!(solution, 31);
    }
}
