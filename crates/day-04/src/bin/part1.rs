#[derive(Debug, PartialEq)]
struct WordSearch;

fn main() {}

fn parse(input: &str) -> WordSearch {
    todo!();
}

fn solve(word_search: &WordSearch) -> usize {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = include_str!("../../input/test");
        let word_search = parse(input);
        assert_eq!(word_search, WordSearch);
    }

    #[test]
    fn test_solve() {
        let input = include_str!("../../input/test");
        let word_search = parse(input);
        let solution = solve(&word_search);
        assert_eq!(solution, 18);
    }
}
