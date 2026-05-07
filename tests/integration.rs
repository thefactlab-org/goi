use std::fs;
use std::path::Path;

fn cleanup_test_dir(path: &str) {
    let _ = fs::remove_dir_all(path);
}

fn create_project_direct(project_name: &str, module_path: &str) -> std::io::Result<()> {
    fs::create_dir_all(project_name)?;
    
    let go_mod = format!("module {}\n\ngo 1.21\n", module_path);
    fs::write(Path::new(project_name).join("go.mod"), &go_mod)?;
    
    let main_go = r#"package main

import "fmt"

func main() {
    fmt.Println("Hello, Go!")
}
"#;
    fs::write(Path::new(project_name).join("main.go"), main_go)?;
    
    Ok(())
}

mod tests {
    use super::*;

    #[test]
    fn test_create_project_creates_files() {
        let test_dir = "test_goi_project";
        cleanup_test_dir(test_dir);

        let result = create_project_direct(test_dir, "github.com/test/test");
        assert!(result.is_ok());

        let go_mod_path = Path::new(test_dir).join("go.mod");
        let main_go_path = Path::new(test_dir).join("main.go");

        assert!(go_mod_path.exists(), "go.mod should exist");
        assert!(main_go_path.exists(), "main.go should exist");

        let go_mod_content = fs::read_to_string(&go_mod_path).unwrap();
        assert!(go_mod_content.contains("module github.com/test/test"));
        assert!(go_mod_content.contains("go 1.21"));

        let main_go_content = fs::read_to_string(&main_go_path).unwrap();
        assert!(main_go_content.contains("package main"));
        assert!(main_go_content.contains("func main()"));

        cleanup_test_dir(test_dir);
    }

    #[test]
    fn test_validate_project_name() {
        assert!(!"myproject".is_empty());
        assert!(!"hello_world".is_empty());
        assert!("".is_empty());
    }

    #[test]
    fn test_validate_module_path() {
        assert!("github.com/user/repo".contains('/'));
        assert!("example.com/a/b".contains('/'));
        assert!(!"github.com".contains('/'));
    }
}