use crate::Budget::Budget;

pub enum State {
    Menu,
    ViewAllBudgets(Vec<Budget>),
    ViewSpecificBudget(Budget),
    Finished,
    AddNewBudget,
    AddExpenditure(Expenditure)
    RemoveBudget(Budget),
}
