use crate::budget::{Budget, Expenditure};

pub enum State {
    Menu,
    ViewAllBudgets(Vec<Budget>),
    ViewSpecificBudget(Budget),
    Finished,
    AddExpenditure(Expenditure),
    RemoveExpenditure(Expenditure),
    EditExpenditure(Expenditure),
    AddNewBudget,
    RemoveBudget(Budget),
    EditBudget(Budget)
}
