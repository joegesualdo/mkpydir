use std::env;
use std::fs;
use std::io::prelude::*;
use std::io::Result;
use std::process;

fn main() -> Result<()> {
    // Content to write to the files
    let args: Vec<String> = env::args().collect();

    // Check if at least one argument is provided (first is the program name)
    if args.len() < 2 {
        println!("Please provide a directory/project name.");
        process::exit(1);
    }
    let directory_name = &args[1];
    let dir_path = directory_name;
    fs::create_dir_all(dir_path)?;

    // Step 2: Create and write to file1.txt

    let conda_environment_yaml_file_content = format!(
        r#"name: {directory_name} 
channels:
    - conda-forge
    - defaults
dependencies:
    - python=3.9
    - conda-forge::numpy=1.21.*
    - nodejs=16.13.*
    - flask
    - pip
    - pip:
        - -r requirements.txt
    "#
    );
    let requirements_file_content = r#"
requests==2.25.1
    "#;

    // Create and write to the first file
    let conda_environment_yaml_file_path = format!("{}/environment.yml", directory_name);
    let mut conda_environment_yaml_file = fs::File::create(&conda_environment_yaml_file_path)?;
    conda_environment_yaml_file.write_all(conda_environment_yaml_file_content.as_bytes())?;

    // Create and write to the second file
    let requirements_file_path = format!("{}/requirements.txt", directory_name);
    let mut requirements_file = fs::File::create(&requirements_file_path)?;
    requirements_file.write_all(requirements_file_content.as_bytes())?;

    Ok(())
}
