// ---
// Project: Kairos
// File: main.rs
// ---

use std::io;

fn kairos_get_input() -> Vec<String> {
    let mut inputs: Vec<String> = Vec::new();
    let mut project_name = String::new();
    let mut github_https = String::new();
    let mut code_language = String::new();

    println!("Project name:");
    io::stdin().read_line(&mut project_name).expect("Failed to read the line");
    println!("Github HTTPS URL (empty to skip):");
    io::stdin().read_line(&mut github_https).expect("Failed to read the line");
    println!("Language: 1.C   2.C++   3.Python   4.Haskell   5.Rust");
    io::stdin().read_line(&mut code_language).expect("Failed to read the line");

    inputs.push(project_name);
    inputs.push(github_https);
    inputs.push(code_language);
    inputs
}

fn main() {
    let _inputs = kairos_get_input();
}
