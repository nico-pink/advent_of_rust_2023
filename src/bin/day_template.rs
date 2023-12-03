use std::{env, fs};

// placeholder file for creating templates
// may add updates to imports, benhcmark functions, invocations in main.rs., etc.

fn to_solution_file(day_num: &String) -> std::io::Result<()> {
    let file_path = format!("src/solution_{day_num}.rs");
    fs::copy("solution_template", file_path)?;
    Ok(())
}

fn add_sample_input_file(day_num: &String) -> std::io::Result<()> {
    let file_path = format!("data_sample/input{day_num}.txt");
    fs::File::create(file_path)?;
    Ok(())
}

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let day_num = &args[1];
    to_solution_file(day_num).unwrap();
    add_sample_input_file(day_num).unwrap();
}
