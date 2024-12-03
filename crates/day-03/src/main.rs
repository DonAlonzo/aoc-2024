#[derive(Debug, PartialEq)]
struct Instruction(u32, u32);

fn main() {
    let instructions = parse(include_str!("../input/real"));
    let solution_part1 = solve_part1(&instructions);
    println!("Part 1: {solution_part1}");
}

fn parse(input: &str) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    for i in 0..input.len() - 3 {
        if &input[i..i + 4] != "mul(" {
            continue;
        }
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
    instructions
}

fn solve_part1(instructions: &[Instruction]) -> u32 {
    instructions
        .iter()
        .fold(0, |acc, Instruction(lhs, rhs)| acc + lhs * rhs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let instructions = parse(include_str!("../input/test"));
        assert_eq!(
            instructions,
            vec![
                Instruction(2, 4),
                Instruction(5, 5),
                Instruction(11, 8),
                Instruction(8, 5),
            ]
        );
    }

    #[test]
    fn test_solve_part1() {
        let instructions = parse(include_str!("../input/test"));
        let solution_part1 = solve_part1(&instructions);
        assert_eq!(solution_part1, 161);
    }
}
