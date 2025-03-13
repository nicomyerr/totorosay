#[cfg(test)]
mod tests {
    use rstest::rstest;
    use std::fs;

    #[rstest]
    #[case("totoro-single-word.txt", "totorosay", false)]
    #[case("totoro-big-single-word.txt", "totorosay", true)]
    fn test_totorosay(#[case] filename: &str, #[case] text: &str, #[case] big: bool) {
        // given
        let expected = read_file(filename);
        // when
        let result = totorosay::totorosay(text.to_string(), big);
        // then
        assert_eq!(result, expected);
    }

    fn read_file(filename: &str) -> String {
        let path = "tests/resources/".to_string();
        return fs::read_to_string(path + &filename).expect("Unable to read file");
    }
}
