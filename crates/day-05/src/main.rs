#[derive(Debug, PartialEq)]
struct Rules(Vec<(u32, u32)>);

#[derive(Debug, PartialEq)]
struct Update(Vec<u32>);

fn main() {
    let input = include_str!("../input/real");
    let (rules, updates) = parse(input);
    let solution_part1 = solve_part1(&rules, &updates);
    let solution_part2 = solve_part2(&rules, &updates);
    println!("Part 1: {solution_part1}");
    println!("Part 2: {solution_part2}");
}

fn parse(input: &str) -> (Rules, Vec<Update>) {
    let (rules, updates) = input.split_once("\n\n").unwrap();
    let rules = rules
        .lines()
        .map(|line| {
            let (lhs, rhs) = line.split_once('|').unwrap();
            (lhs.parse().unwrap(), rhs.parse().unwrap())
        })
        .collect();
    let updates = updates
        .lines()
        .map(|line| {
            let pages = line.split(',').map(|page| page.parse().unwrap()).collect();
            Update(pages)
        })
        .collect();
    (Rules(rules), updates)
}

fn solve_part1(rules: &Rules, updates: &[Update]) -> u32 {
    updates
        .iter()
        .filter(|update| {
            if update.0.len() < 2 {
                return true;
            }
            update
                .0
                .windows(2)
                .all(|pages| rules.before(pages[0], pages[1]))
        })
        .map(|update| update.0[update.0.len() / 2])
        .sum()
}

fn solve_part2(rules: &Rules, updates: &[Update]) -> u32 {
    updates
        .iter()
        .filter(|update| {
            if update.0.len() < 2 {
                return false;
            }
            update
                .0
                .windows(2)
                .any(|pages| !rules.before(pages[0], pages[1]))
        })
        .map(|update| update.reorder(rules))
        .map(|update| update.0[update.0.len() / 2])
        .sum()
}

impl Rules {
    fn before(&self, a: u32, b: u32) -> bool {
        self.0.iter().all(|(lhs, rhs)| a != *rhs || b != *lhs)
    }
}

impl Update {
    fn reorder(&self, rules: &Rules) -> Update {
        let mut update = self.0.clone();
        for i in 0..update.len() - 1 {
            if !rules.before(update[i], update[i + 1]) {
                update.swap(i, i + 1)
            }
        }
        Update(update)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = include_str!("../input/test");
        let (rules, updates) = parse(input);
        assert!(rules.before(47, 53));
        assert!(rules.before(97, 13));
        assert!(rules.before(97, 61));
        assert!(rules.before(97, 47));
        assert!(rules.before(75, 29));
        assert!(rules.before(61, 13));
        assert!(rules.before(75, 53));
        assert!(rules.before(29, 13));
        assert!(rules.before(97, 29));
        assert!(rules.before(53, 29));
        assert!(rules.before(61, 53));
        assert!(rules.before(97, 53));
        assert!(rules.before(61, 29));
        assert!(rules.before(47, 13));
        assert!(rules.before(75, 47));
        assert!(rules.before(97, 75));
        assert!(rules.before(47, 61));
        assert!(rules.before(75, 61));
        assert!(rules.before(47, 29));
        assert!(rules.before(75, 13));
        assert!(rules.before(53, 13));
        assert_eq!(
            updates,
            vec![
                Update(vec![75, 47, 61, 53, 29]),
                Update(vec![97, 61, 53, 29, 13]),
                Update(vec![75, 29, 13]),
                Update(vec![75, 97, 47, 61, 53]),
                Update(vec![61, 13, 29]),
                Update(vec![97, 13, 75, 29, 47]),
            ]
        )
    }

    #[test]
    fn test_solve_part1() {
        let input = include_str!("../input/test");
        let (rules, updates) = parse(input);
        assert_eq!(solve_part1(&rules, &updates), 143);
    }

    #[test]
    fn test_reorder() {
        let input = include_str!("../input/test");
        let (rules, _) = parse(input);
        assert_eq!(
            Update(vec![75, 97, 47, 61, 53]).reorder(&rules),
            Update(vec![97, 75, 47, 61, 53]),
        );
        assert_eq!(
            Update(vec![61, 13, 29]).reorder(&rules),
            Update(vec![61, 29, 13]),
        );
        assert_eq!(
            Update(vec![97, 13, 75, 29, 47]).reorder(&rules),
            Update(vec![97, 75, 47, 29, 13]),
        );
    }
}
