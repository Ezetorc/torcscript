use colored::Colorize;
use std::fmt::Display;

pub fn print_vector<Item: Display>(vector: &[Item], prefix: &str) {
    print!("{}", format!("{prefix}: ").green().bold());
    print!("[");

    for (index, item) in vector.iter().enumerate() {
        if index > 0 {
            print!(", ");
        }

        print!("{}", item);
    }

    println!("]");
}
