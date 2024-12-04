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
            count += word_search.search(x, y, &['X', 'M', 'A', 'S']);
        }
    }
    count
}

impl WordSearch {
    fn search(&self, x: usize, y: usize, word: &[char]) -> usize {
        if self.get(x, y) != Some(word[0]) {
            return 0;
        }
        word.iter()
            .enumerate()
            .fold([true; 8], |mut acc, (distance, c)| {
                acc[0] &= self.get(x, y + distance) == Some(*c);
                acc[1] &= self.get(x + distance, y + distance) == Some(*c);
                acc[2] &= self.get(x + distance, y) == Some(*c);
                acc[3] &= self.get(x + distance, y.wrapping_sub(distance)) == Some(*c);
                acc[4] &= self.get(x, y.wrapping_sub(distance)) == Some(*c);
                acc[5] &= self.get(x.wrapping_sub(distance), y.wrapping_sub(distance)) == Some(*c);
                acc[6] &= self.get(x.wrapping_sub(distance), y) == Some(*c);
                acc[7] &= self.get(x.wrapping_sub(distance), y + distance) == Some(*c);
                acc
            })
            .iter()
            .filter(|&&b| b)
            .count()
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
        assert_eq!(solution, 18);
    }
}
