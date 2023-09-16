use std::str::FromStr;

pub struct Budget {
    amount_in_lowest_form: i32,
    currency: Currency,
    name: String,
}

impl Budget {
    fn new(amount_in_lowest_form: i32, currency: Currency, name: String) -> Budget {
        Budget {
            amount_in_lowest_form,
            currency,
            name,
        }
    }
}

//TODO: Add more with support
pub enum SupportedCurrencies {
    Pounds,
}

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

pub struct Expenditure {
    amount_in_lowest_form: i32,
    currency: Currency,
}

impl Expenditure {
    pub fn new(amount_in_lowest_form: i32, currency: Currency) -> Expenditure {
        Expenditure {
            amount_in_lowest_form,
            currency,
        }
    }
}

///Parses the lowest form of the currency into a string form -- e.g. pennies from pounds or cents from dollars
pub fn parse_lowest_form_to_string(input: &str, currency_type: SupportedCurrencies) -> String {
    match currency_type {
        SupportedCurrencies::Pounds => input
            .chars()
            .filter(|c| c.to_owned() != '.'.to_owned() || c.is_numeric())
            .into_iter()
            .collect(),
    }
}

///Parses the lowest form of the currency into an i32 form -- e.g. pennies from pounds or cents from dollars
pub fn parse_lowest_form(
    input: &str,
    currency_type: SupportedCurrencies,
) -> anyhow::Result<i32, <i32 as FromStr>::Err> {
    let string_format = parse_lowest_form_to_string(input, currency_type);
    string_format.trim().parse()
}
