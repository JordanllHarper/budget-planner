use super::currency::Currency;

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

pub struct Expenditure {
    pub amount_in_lowest_form: i32,
    pub currency: Currency,
}

impl Expenditure {
    pub fn new(amount_in_lowest_form: i32, currency: Currency) -> Expenditure {
        Expenditure {
            amount_in_lowest_form,
            currency,
        }
    }
}
