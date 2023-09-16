mod Budget;
mod program_state;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::Budget;

    #[test]
    fn test_parse_lowest_form() {
        let data = "£3.50";
        let expected = 350;
        let actual = Budget
    }
}
