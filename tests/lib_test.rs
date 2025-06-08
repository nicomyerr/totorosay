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
    #[case(
        "totoro-linebreak.txt",
        "Some sentence with a linebreak for testing it.",
        false
    )]
    #[case(
        "totoro-multiple-linebreaks.txt",
        "Do you know the difference between an error and a mistake? Anyone can make an error, but that error doesn't become a mistake until you refuse to correct it.",
        false
    )]
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
        fs::read_to_string(path + filename).expect("Unable to read file")
    }
}
