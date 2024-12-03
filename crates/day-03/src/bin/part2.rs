#[derive(Debug, PartialEq)]
struct Instruction(u32, u32);

fn main() {
    let instructions = parse(include_str!("../../input/real"));
    let solution = solve(&instructions);
    println!("{solution}");
}

fn parse(input: &str) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    let mut do_ = true;
    for i in 0..input.len() - 3 {
        let Some(left_parenthesis_ix) = input[i..].find('(') else {
            break;
        };
        let instruction = &input[i..i + left_parenthesis_ix];
        match instruction {
            "mul" if do_ => {
                let Some(comma_ix) = input[i..].find(',') else {
                    break;
                };
                let Ok(lhs) = input[i + 4..i + comma_ix].parse() else {
                    continue;
                };
                let Some(right_parenthesis_ix) = input[i..].find(')') else {
                    break;
                };
                let Ok(rhs) = input[i + comma_ix + 1..i + right_parenthesis_ix].parse() else {
                    continue;
                };
                instructions.push(Instruction(lhs, rhs));
            }
            "do" => {
                if &input[i + 3..i + 4] != ")" {
                    continue;
                };
                do_ = true;
            }
            "don't" => {
                if &input[i + 6..i + 7] != ")" {
                    continue;
                };
                do_ = false;
            }
            _ => continue,
        }
    }
    instructions
}

fn solve(instructions: &[Instruction]) -> u32 {
    instructions
        .iter()
        .fold(0, |acc, Instruction(lhs, rhs)| acc + lhs * rhs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let instructions = parse(include_str!("../../input/test_part2"));
        assert_eq!(instructions, vec![Instruction(2, 4), Instruction(8, 5),]);
    }

    #[test]
    fn test_solve() {
        let instructions = parse(include_str!("../../input/test_part2"));
        let solution = solve(&instructions);
        assert_eq!(solution, 48);
    }
}
