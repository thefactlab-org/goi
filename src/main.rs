use colored::Colorize;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

fn main() {
    println!("{} {}\n", "Go Project Initializer".white().bold(), "v1".white());

    // Step 1: Project name
    print!("{} ", "📁 Project name:".cyan().bold());
    io::stdout().flush().unwrap();
    let mut project_name = String::new();
    io::stdin().read_line(&mut project_name).unwrap();
    let project_name = project_name.trim();

    if project_name.is_empty() {
        eprintln!("{} Project name cannot be empty", "✗".red());
        std::process::exit(1);
    }

    // Step 2: Module path
    println!(
        "\n{} {}",
        "ℹ Module path format:".yellow(),
        "github.com/username/project".white()
    );
    let default_module = format!("github.com/yourusername/{}", project_name);
    print!("{} [{}]: ", "📦 Module path:".cyan().bold(), default_module.white());
    io::stdout().flush().unwrap();
    let mut module_path = String::new();
    io::stdin().read_line(&mut module_path).unwrap();
    let module_path = module_path.trim();
    
    let module_path = if module_path.is_empty() {
        &default_module
    } else {
        module_path
    };

    // Step 3: Confirm
    println!("\n{} {}", "📋 Summary:".green().bold(), "Confirm before creating".white());
    println!("   Project: {}", project_name.cyan());
    println!("   Module:  {}", module_path.blue());
    print!("\n{} [Y/n]: ", "Create?".green().bold());
    io::stdout().flush().unwrap();
    let mut confirm = String::new();
    io::stdin().read_line(&mut confirm).unwrap();
    
    if confirm.trim().to_lowercase() != "y" && confirm.trim() != "" {
        println!("{} Cancelled", "✗".red());
        std::process::exit(0);
    }

    // Create project
    if let Err(e) = create_project(project_name, module_path) {
        eprintln!("{} {}", "✗ Error:".red().bold(), e);
        std::process::exit(1);
    }
}

fn create_project(project_name: &str, module_path: &str) -> io::Result<()> {
    println!("\n{}", "Creating project...".green());

    // Create project directory
    fs::create_dir_all(project_name)?;
    
    // Create go.mod
    let go_mod = format!(
        "module {}\n\ngo 1.21\n",
        module_path
    );
    fs::write(Path::new(project_name).join("go.mod"), &go_mod)?;
    println!("{} go.mod", "   ✓".green());
    
    // Create main.go
    let main_go = r#"package main

import "fmt"

func main() {
    fmt.Println("Hello, Go!")
}
"#;
    fs::write(Path::new(project_name).join("main.go"), main_go)?;
    println!("{} main.go", "   ✓".green());
    
    println!(
        "\n{} {}\n",
        "✅ Success!".green().bold(),
        format!("cd {} && go run main.go", project_name).white()
    );
    
    Ok(())
}
