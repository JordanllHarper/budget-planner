use crate::Budget::Budget;

pub enum State {
    Menu,
    ViewAllBudgets(Vec<Budget>),
    ViewSpecificBudget(Budget),
    Finished,
    AddNewBudget,
    RemoveBudget(Budget),
}
