# Lời Giải Chương 5

## Bài 1: Chia an toàn

```rust
fn chia(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Không thể chia cho 0!"))  // Err = lỗi
    } else {
        Ok(a / b)                                    // Ok = thành công
    }
}

fn main() {
    match chia(10.0, 3.0) {
        Ok(kq) => println!("10/3 = {:.2}", kq),
        Err(e) => println!("Lỗi: {}", e),
    }
    match chia(10.0, 0.0) {
        Ok(kq) => println!("10/0 = {:.2}", kq),
        Err(e) => println!("Lỗi: {}", e),            // In: Lỗi: Không thể chia cho 0!
    }
}
```

## Bài 2: Parse config

```rust
fn parse_config(line: &str) -> Result<(String, String), String> {
    let parts: Vec<&str> = line.splitn(2, '=').collect();  // Tách tại '=' đầu tiên
    if parts.len() != 2 {
        Err(format!("Dòng không hợp lệ: '{}'", line))     // Không có '='
    } else {
        Ok((parts[0].trim().to_string(), parts[1].trim().to_string()))
    }
}

fn main() {
    let lines = ["host=localhost", "port=8080", "invalid line"];
    for line in lines {
        match parse_config(line) {
            Ok((key, val)) => println!("  {} → {}", key, val),
            Err(e) => println!("  ❌ {}", e),
        }
    }
}
```

## Bài 3: Chuỗi ? operator

```rust
use std::fs;

fn doc_va_tinh(path: &str) -> Result<f64, Box<dyn std::error::Error>> {
    let noi_dung = fs::read_to_string(path)?;          // ? = return Err nếu lỗi đọc file
    let mut tong = 0.0;
    let mut dem = 0;
    for line in noi_dung.lines() {                     // Duyệt từng dòng
        let so: f64 = line.trim().parse()?;            // ? = return Err nếu parse lỗi
        tong += so;
        dem += 1;
    }
    if dem == 0 {
        Err("File rỗng!".into())                       // .into() chuyển &str → Box<dyn Error>
    } else {
        Ok(tong / dem as f64)                          // Trung bình
    }
}

fn main() {
    match doc_va_tinh("numbers.txt") {
        Ok(tb) => println!("Trung bình: {:.2}", tb),
        Err(e) => println!("Lỗi: {}", e),
    }
}
```
