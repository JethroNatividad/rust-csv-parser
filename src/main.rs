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
fn main() {
    let contents =
        std::fs::read_to_string("src/input.csv").expect("Should have been able to read the file");

    // Split newlines
    // Split commas - lastname, firstname, salary

    contents.split("\n").for_each(|line| println!("{}", line));

    // println!("{}", contents);
}
