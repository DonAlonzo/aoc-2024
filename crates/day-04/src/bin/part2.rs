#[derive(Debug, PartialEq)]
struct WordSearch {
    letters: Vec<char>,
    width: usize,
    height: usize,
}

fn main() {
    let input = include_str!("../../input/real");
    let word_search = parse(input);
    let solution = solve(&word_search);
    println!("{solution}");
}

fn parse(input: &str) -> WordSearch {
    let letters = input.chars().filter(|c| !c.is_whitespace()).collect();
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    WordSearch {
        letters,
        width,
        height,
    }
}

fn solve(word_search: &WordSearch) -> usize {
    let mut count = 0;
    for x in 0..word_search.width {
        for y in 0..word_search.height {
            if word_search.search(x, y) {
                count += 1;
            }
        }
    }
    count
}

impl WordSearch {
    fn search(&self, x: usize, y: usize) -> bool {
        if x == 0 || y == 0 || x == self.width - 1 || y == self.height - 1 {
            return false;
        }
        if self.get(x, y) != Some('A') {
            return false;
        }
        let upper_left = self.get(x - 1, y - 1);
        let upper_right = self.get(x + 1, y - 1);
        let lower_right = self.get(x + 1, y + 1);
        let lower_left = self.get(x - 1, y + 1);
        let x = (upper_left, upper_right, lower_right, lower_left);
        let mmss = (Some('M'), Some('M'), Some('S'), Some('S'));
        let mssm = (Some('M'), Some('S'), Some('S'), Some('M'));
        let ssmm = (Some('S'), Some('S'), Some('M'), Some('M'));
        let smms = (Some('S'), Some('M'), Some('M'), Some('S'));
        x == mmss || x == mssm || x == ssmm || x == smms
    }

    fn get(&self, x: usize, y: usize) -> Option<char> {
        if x < self.width && y < self.height {
            Some(self.letters[y * self.width + x])
        } else {
            None
        }
    }
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
                letters: vec![
                    'M', 'M', 'M', 'S', 'X', 'X', 'M', 'A', 'S', 'M', 'M', 'S', 'A', 'M', 'X', 'M',
                    'S', 'M', 'S', 'A', 'A', 'M', 'X', 'S', 'X', 'M', 'A', 'A', 'M', 'M', 'M', 'S',
                    'A', 'M', 'A', 'S', 'M', 'S', 'M', 'X', 'X', 'M', 'A', 'S', 'A', 'M', 'X', 'A',
                    'M', 'M', 'X', 'X', 'A', 'M', 'M', 'X', 'X', 'A', 'M', 'A', 'S', 'M', 'S', 'M',
                    'S', 'A', 'S', 'X', 'S', 'S', 'S', 'A', 'X', 'A', 'M', 'A', 'S', 'A', 'A', 'A',
                    'M', 'A', 'M', 'M', 'M', 'X', 'M', 'M', 'M', 'M', 'M', 'X', 'M', 'X', 'A', 'X',
                    'M', 'A', 'S', 'X',
                ],
                width: 10,
                height: 10,
            }
        );
    }

    #[test]
    fn test_solve() {
        let input = include_str!("../../input/test");
        let word_search = parse(input);
        let solution = solve(&word_search);
        assert_eq!(solution, 9);
    }
}
