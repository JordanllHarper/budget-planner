use crate::Budget::Currency;

pub struct Expenditure {
    amount_in_lowest_form: i32,
    currency: Currency,
    label: String,
}
