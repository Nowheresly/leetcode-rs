use std::collections::HashMap;

struct Spreadsheet {
    cells: HashMap<String, i32>
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Spreadsheet {

    fn new(_rows: i32) -> Self {
        Self {
            cells: HashMap::new()
        }
    }

    fn set_cell(&mut self, cell: String, value: i32) {
        self.cells.insert(cell, value);
    }

    fn reset_cell(&mut self, cell: String) {
        self.cells.remove(&cell);
    }

    fn get_value(&self, formula: String) -> i32 {
        let idx_plus = formula.find('+').unwrap();
        let op1 = &formula[1..idx_plus];
        let op2 = &formula[idx_plus+1..formula.len()];
        let v1 = i32::from_str_radix(&op1, 10).unwrap_or_else(|_| *self.cells.get(op1).unwrap_or(&0));
        let v2 = i32::from_str_radix(&op2, 10).unwrap_or_else(|_| *self.cells.get(op2).unwrap_or(&0));
        v1 + v2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut spread = Spreadsheet::new(3);
        assert_eq!(12, spread.get_value(String::from("=5+7")));
        spread.set_cell(String::from("A1"), 10);
        assert_eq!(16, spread.get_value(String::from("=A1+6")));
        spread.set_cell(String::from("B2"), 15);
        assert_eq!(25, spread.get_value(String::from("=A1+B2")));
        spread.reset_cell(String::from("A1"));
        assert_eq!(15, spread.get_value(String::from("=A1+B2")));
    }
}
