use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

const VALUE: &str = "VALUE";
const ADD: &str = "ADD";
const SUB: &str = "SUB";
const MUL: &str = "MULT";

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, usize);
    let mut spreadsheet = Spreadsheet::new(read_operations(n));
    spreadsheet.fill_spreadsheet();

    for cell in spreadsheet.cells {
        // Write an answer using println!("message...");
        // To debug: eprintln!("Debug message...");

        println!("{}", cell.unwrap());
    }
}

fn read_operations(count: usize) -> Vec<Operation> {
    let mut cells: Vec<Operation> = Vec::with_capacity(count);

    for i in 0..count as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();

        cells.push(Operation {
            command: inputs[0].trim().to_string(),
            value_one: inputs[1].trim().to_string(),
            value_two: inputs[2].trim().to_string(),
            cell_number: i,
        });
    }

    return cells;
}

struct Spreadsheet {
    operations: Vec<Operation>,
    cells: Vec<Option<i32>>,
}

impl Spreadsheet {
    fn new(operations: Vec<Operation>) -> Spreadsheet {
        let length = operations.len();
        return Spreadsheet {
            operations,
            cells: (0..length).map(|_| None).collect(),
        };
    }

    fn fill_spreadsheet(&mut self) {
        while !self.operations.is_empty() {
            let operation = self.operations.swap_remove(0);

            self.apply_operation(&operation);
        }
    }

    fn get_value(&mut self, raw_value: &str) -> i32 {
        if raw_value.starts_with('$') {
            return self.get_value_from_ref(raw_value);
        }

        return raw_value.parse::<i32>().unwrap();
    }

    fn get_value_from_ref(&mut self, raw_value: &str) -> i32 {
        let index = raw_value[1..].parse::<usize>().unwrap();

        let cell_value = &self.cells[index];
        if cell_value.is_some() {
            return cell_value.unwrap();
        }

        let cell_operation_index = self
            .operations
            .iter()
            .position(|o| o.cell_number == index)
            .unwrap();

        let cell_operation = self.operations.remove(cell_operation_index);
        self.apply_operation(&cell_operation);

        return self.cells[index].unwrap();
    }

    fn apply_operation(&mut self, operation: &Operation) {
        let value = self.get_operation_cell_value(operation);
        if value.is_none() {
            return;
        }

        self.cells[operation.cell_number].insert(value.unwrap());
    }

    fn get_operation_cell_value(&mut self, operation: &Operation) -> Option<i32> {
        let one = self.get_value(&operation.value_one);

        if operation.command == VALUE {
            return Some(one);
        }

        let two = self.get_value(&operation.value_two);

        return match operation.command.as_str() {
            ADD => Some(one + two),
            SUB => Some(one - two),
            MUL => Some(one * two),
            _ => None,
        };
    }
}

struct Operation {
    command: String,
    value_one: String,
    value_two: String,
    cell_number: usize,
}

impl Operation {
    fn get_values(&self, cells: &Vec<Option<i32>>) -> (Option<i32>, Option<i32>) {
        return (
            self.get_value(&self.value_one, cells),
            self.get_value(&self.value_two, cells),
        );
    }

    fn get_value(&self, raw_value: &str, cells: &Vec<Option<i32>>) -> Option<i32> {
        if raw_value == "_" {
            return None;
        }

        if raw_value.starts_with('$') {
            let index = raw_value[1..].parse::<usize>().unwrap();
            return cells[index];
        }

        return Some(raw_value.parse::<i32>().unwrap());
    }
}
