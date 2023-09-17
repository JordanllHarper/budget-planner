use crate::budget::SupportedCurrencies;

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