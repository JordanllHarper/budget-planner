use crate::models::budget::{self, Budget};

pub struct BudgetCollection {
    pub list: Vec<Budget>,
}

impl BudgetCollection {
    pub fn new() -> Self {
        Self { list: Vec::new() }
    }
    pub fn add(mut self, budget: Budget) -> Self {
        self.list.push(budget);
        self
    }
}
