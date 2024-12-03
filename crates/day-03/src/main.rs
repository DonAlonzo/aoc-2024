fn main() {}

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
}
