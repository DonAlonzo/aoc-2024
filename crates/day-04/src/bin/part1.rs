#[derive(Debug, PartialEq)]
struct WordSearch<'a> {
    letters: &'a [char],
    width: usize,
}

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
        assert_eq!(
            word_search,
            WordSearch {
                letters: &[
                    'M', 'M', 'M', 'S', 'X', 'X', 'M', 'A', 'S', 'M', 'M', 'S', 'A', 'M', 'X', 'M',
                    'S', 'M', 'S', 'A', 'A', 'M', 'X', 'S', 'X', 'M', 'A', 'A', 'M', 'M', 'M', 'S',
                    'A', 'M', 'A', 'S', 'M', 'S', 'M', 'X', 'X', 'M', 'A', 'S', 'A', 'M', 'X', 'A',
                    'M', 'M', 'X', 'X', 'A', 'M', 'M', 'X', 'X', 'A', 'M', 'A', 'S', 'M', 'S', 'M',
                    'S', 'A', 'S', 'X', 'S', 'S', 'S', 'A', 'X', 'A', 'M', 'A', 'S', 'A', 'A', 'A',
                    'M', 'A', 'M', 'M', 'M', 'X', 'M', 'M', 'M', 'M', 'M', 'X', 'M', 'X', 'A', 'X',
                    'M', 'A', 'S', 'X',
                ],
                width: 10,
            }
        );
    }

    #[test]
    fn test_solve() {
        let input = include_str!("../../input/test");
        let word_search = parse(input);
        let solution = solve(&word_search);
        assert_eq!(solution, 18);
    }
}
