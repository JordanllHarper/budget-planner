#[cfg(test)]
mod tests {
    use crate::currency::{parse_lowest_form, SupportedCurrencies};

    #[test]
    fn test_parse_lowest_form() {
        let data = "£3.50";
        let expected = 350;
        let actual = parse_lowest_form(data, SupportedCurrencies::Pounds).expect("Test data");

        assert_eq!(expected, actual)
    }
}
