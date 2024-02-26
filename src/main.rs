use cli_table::{print_stdout, Table, WithTitle};
// Create a program that reads a csv and outputs a table
/// Inputs: csv file with:
// Ling,Mai,55900
// Johnson,Jim,56500
// Jones,Aaron,46000
// Jones,Chris,34500
// Swift,Geoffrey,14200
// Xiong,Fong,65000
// Zarnecki,Sabrina,51500
// Process: make this into a vector of structs
// Outputs: table

#[derive(Table)]
struct Employee {
    #[table(title = "First Name")]
    first_name: String,
    #[table(title = "Last Name")]
    last_name: String,
    #[table(title = "Salary")]
    salary: String,
}

fn main() {
    let contents =
        std::fs::read_to_string("src/input.csv").expect("Should have been able to read the file");
    let mut employees: Vec<Employee> = vec![];

    // Split newlines
    // Split commas - lastname, firstname, salary

    contents.split("\n").for_each(|line| {
        let mut data: Vec<&str> = vec![];
        line.split(",").for_each(|d| data.push(d));
        employees.push(Employee {
            last_name: data.get(0).unwrap().to_string(),
            first_name: data.get(1).unwrap().to_string(),
            salary: data.get(2).unwrap().to_string(),
        })
    });

    assert!(print_stdout(employees.with_title()).is_ok())
    // println!("{}", contents);
}
