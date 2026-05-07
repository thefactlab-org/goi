use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
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

fn benchmark_create_project(c: &mut Criterion) {
    c.bench_function("create_project_single", |b| {
        b.iter(|| {
            let test_dir = "bench_goi_project";
            cleanup_test_dir(test_dir);
            let _ = create_project_direct(black_box(test_dir), black_box("github.com/test/test"));
            cleanup_test_dir(test_dir);
        })
    });
}

fn benchmark_multiple_creations(c: &mut Criterion) {
    let mut group = c.benchmark_group("create_project_iterations");
    
    for i in [5, 10, 20].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(i), i, |b, &i| {
            b.iter(|| {
                for j in 0..i {
                    let test_dir = format!("bench_project_{}", j);
                    cleanup_test_dir(&test_dir);
                    let _ = create_project_direct(&test_dir, "github.com/test/bench");
                    cleanup_test_dir(&test_dir);
                }
            })
        });
    }
    group.finish();
}

criterion_group!(benches, benchmark_create_project, benchmark_multiple_creations);
criterion_main!(benches);