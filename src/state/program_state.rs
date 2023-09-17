use crate::budget_repo::BudgetCollection;
use crate::models::budget::Budget;

pub enum State {
    Menu,
    ViewAllBudgets(BudgetCollection),
    ViewSpecificBudget(BudgetCollection),
    Finished,
    AddNewBudget(BudgetCollection),
    RemoveBudget(BudgetCollection),
    EditBudget(BudgetCollection),
}

pub enum ExpenditureState {
    AddExpenditure(Budget),
    RemoveExpenditure(Budget),
    EditExpenditure(Budget),
}
