use std::str::FromStr;

pub struct Currency {
    currency_type: SupportedCurrencies,
    sign: String,
}
impl Currency {
    pub fn new(currency_type: SupportedCurrencies, sign: String) -> Currency {
        Currency {
            currency_type,
            sign,
        }
    }
}

pub enum SupportedCurrencies {
    Pounds,
}
///Parses the lowest form of the currency into a string form -- e.g. pennies from pounds or cents from dollars
pub fn parse_lowest_form_to_string(input: &str, currency_type: SupportedCurrencies) -> String {
    match currency_type {
        SupportedCurrencies::Pounds => input
            .chars()
            .filter(|c| c.is_numeric())
            .into_iter()
            .collect(),
    }
}

///Parses the lowest form of the currency into an i32 form -- e.g. pennies from pounds or cents from dollars
pub fn parse_lowest_form(
    input: &str,
    currency_type: SupportedCurrencies,
) -> anyhow::Result<i32, <i32 as FromStr>::Err> {
    parse_lowest_form_to_string(input, currency_type)
        .trim()
        .parse()
}
