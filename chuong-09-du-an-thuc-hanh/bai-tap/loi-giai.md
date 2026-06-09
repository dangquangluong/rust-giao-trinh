# Lời Giải Chương 9: Starter Code

## Dự Án 1: Thống kê

```rust
fn main() {
    let data = vec![85.0, 90.0, 78.0, 92.0, 88.0, 95.0, 67.0, 73.0];

    let n = data.len() as f64;
    let tong: f64 = data.iter().sum();
    let tb = tong / n;

    let mut sorted = data.clone();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let median = sorted[sorted.len() / 2];

    let max = data.iter().cloned().fold(f64::MIN, f64::max);
    let min = data.iter().cloned().fold(f64::MAX, f64::min);

    let variance: f64 = data.iter().map(|x| (x - tb).powi(2)).sum::<f64>() / n;
    let std_dev = variance.sqrt();

    println!("Số lượng: {}", data.len());
    println!("Tổng: {:.1}", tong);
    println!("TB: {:.2}, Median: {:.1}", tb, median);
    println!("Max: {}, Min: {}", max, min);
    println!("Độ lệch chuẩn: {:.2}", std_dev);
}
```

## Dự Án 2: Mini Grep (starter)

```rust
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Dùng: minigrep <pattern> <file>");
        std::process::exit(1);
    }
    let pattern = &args[1];
    let content = fs::read_to_string(&args[2]).expect("Lỗi đọc file");

    for (i, line) in content.lines().enumerate() {
        if line.contains(pattern.as_str()) {
            println!("\x1b[32m{}:\x1b[0m {}", i + 1, line);  // In màu xanh
        }
    }
}
```

*(Xem Chương 9 README cho code Todo App đầy đủ)*
