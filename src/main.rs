use std::io::{stdin, stdout};

use budget_repo::BudgetCollection;
use state::program_state::{self, State};

mod models;
mod state;
mod tests;

mod budget_repo;
fn main() {
    let mut program_state = state::program_state::State::Menu;

    loop {
        program_state = match program_state {
            program_state::State::Menu => menu(),
            program_state::State::ViewAllBudgets(_) => todo!(),
            //This should take you to a manage expenditures state
            program_state::State::ViewSpecificBudget(_) => todo!(),
            program_state::State::Finished => break,
            program_state::State::AddNewBudget(_) => todo!(),
            program_state::State::RemoveBudget(_) => todo!(),
            program_state::State::EditBudget(_) => todo!(),
        }
    }
}

fn menu(budget_collection: BudgetCollection) -> State {
    //options
    let view_budgets = "1";
    let view_specific_budg = "2";
    let add_new_budget = "3";
    let remove_budget = "4";
    let edit_budget = "5";

    let finished = "6";
    println!(
        r"Press {view_budgets} to view budgets.
    Press {view_specific_budg} with the number provided to view a specific budget.
    Press {add_new_budget} to add a new budget.
    Press {remove_budget} to remove a budget.
    Press {edit_budget} to edit aspects of a budget.
    Press {finished} to exit.
    "
    );

    let mut input_buf = String::new();

    let _ = stdin().read_line(&mut input_buf);
    match input_buf.trim() {
        view_budgets => State::ViewAllBudgets(budget_collection),
        _ => State::Menu,
    }
}
