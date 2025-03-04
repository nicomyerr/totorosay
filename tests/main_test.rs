#[cfg(test)]
mod tests {
    use rstest::rstest;
    use totorosay::size_to_file;

    #[rstest]
    #[case(true, "totoro-big.txt")]
    #[case(false, "totoro.txt")]
    fn test_size_to_file(#[case] input: bool, #[case] expected: &str) {
        let result = size_to_file(input);
        assert_eq!(result, expected);
    }
}
