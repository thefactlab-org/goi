# goi

**Go Init** - Quickly scaffold Go projects

## Installation

### From Download (macOS/arm64)
```bash
curl -LO https://github.com/thefactlab-org/goi/releases/latest/download/goi_darwin_arm64
```
or
```bash
curl -sSL https://raw.githubusercontent.com/thefactlab-org/goi/main/download/goi_darwin_arm64 -o goi
```

### Add to PATH (choose one)
**macOS/Linux:**
```bash
cp target/release/goi /usr/local/bin/
```

**Or** add an alias in `~/.zshrc` or `~/.bashrc`:
```bash
alias goi='/path/to/goi/target/release/goi'
```

**Windows:**
```powershell
copy target\release\goi.exe C:\Windows\System32\goi.exe
```

### From Source
```bash
cd goi
cargo build --release
```

The executable will be at `target/release/goi`

## Usage

```bash
goi
```

### Steps:
1. Enter project name
2. Enter module path (or press Enter for default)
3. Confirm creation (Y/n, or press Enter for Y default))

### Example:
```
🚀 Go Project Initializer v0.1.0

📁 Project name: myproject

ℹ Module path format: github.com/username/project
📦 Module path [github.com/yourusername/myproject]: github.com/name/myproject

📋 Summary: Confirm before creating
   Project: myproject
   Module:  github.com/name/myproject

Create? [Y/n]: Y

Creating project...
   ✓ go.mod
   ✓ main.go

✅ Success! cd myproject && go run main.go
```

## Testing

> **🧪 Test Environment**
> - Device: MacBook Air M4, RAM 16GB
> - All tests and benchmarks were run on this configuration

### Run Integration Tests
```bash
cargo test
```

### Test Output:
```
running 3 tests
test tests::test_validate_module_path ... ok
test tests::test_validate_project_name ... ok
test tests::test_create_project_creates_files ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## Benchmarking

### Run Benchmarks
```bash
cargo bench
```

### ⏱ Project Creation Speed

| Test | Time |
|------|------|
| Create 1 project | ~251-281 µs |
| Create 5 projects (total) | ~1.13-1.18 ms |
| Create 10 projects (total) | ~2.27-2.47 ms |
| Create 20 projects (total) | ~4.40-4.62 ms |

### 📊 Performance Overview

```
Create 1 project:
███████████████████████████████████████ ~260 µs

Create 5 projects (total):
██████████████████████████████████████████████████████████████████████████████ ~1.15 ms

Create 10 projects (total):
████████████████████████████████████████████████████████████████████████████████████████████████████ ~2.35 ms

Create 20 projects (total):
████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████ ~4.50 ms
```

### 📈 Comparison with Manual Methods

| Method | Time (average) |
|--------|----------------|
| goi (this tool) | ~260 µs |
| Manual (mkdir + echo) | ~2-5 ms |
| Manual (using editor) | ~5-30 seconds |

```
Relative Speed:

goi           ████████████████████████████████████████████████████ 100%
Manual echo   ████ 5%
Manual edit   █ 1%
```

### Understanding Benchmark Output

```
create_project_single   time:   [251.52 µs 264.52 µs 281.18 µs]
                        change: [-2.6546% +1.6917% +7.4870%] (p = 0.47 > 0.05)
                        No change in performance detected.
```

- **time**: 95% confidence interval for execution time
- **change**: Performance change compared to previous run
- **p < 0.05**: Statistically significant change
- **outliers**: Measurements that deviate from the norm

## Project Structure
```
goi/
├── Cargo.toml           # Project configuration
├── src/
│   └── main.rs          # Main source code
├── tests/
│   └── integration.rs   # Integration tests
├── benches/
│   └── benchmark.rs     # Benchmarks (using criterion)
└── README.md            # This file
```

## Requirements
- [Rust](https://www.rust-lang.org/) (Cargo)
- [Go](https://go.dev/) (for running generated projects)
