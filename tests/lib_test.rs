#[cfg(test)]
mod tests {
    use rstest::rstest;
    use std::fs;

    #[rstest]
    #[case("totoro-single-word.txt", "totorosay", false)]
    #[case("totoro-big-single-word.txt", "totorosay", true)]
    #[case(
        "totoro-max-single-line-single-word.txt",
        "012345678901234567890123456789012345678",
        false
    )]
    fn test_totorosay(#[case] filename: &str, #[case] text: &str, #[case] big: bool) {
        // given
        let expected: String = read_file(filename);
        // when
        let result: String = totorosay::totorosay(text.to_string(), big);
        // then
        assert_eq!(result, expected);
    }

    fn read_file(filename: &str) -> String {
        let path: String = "tests/resources/".to_string();
        return fs::read_to_string(path + &filename).expect("Unable to read file");
    }
}
