# Chương 5: Xử Lý Lỗi (Error Handling)

## 5.1 panic! vs Result

| Loại | Khi nào dùng |
|------|-------------|
| `panic!` | Lỗi nghiêm trọng, không thể tiếp tục |
| `Result<T, E>` | Lỗi có thể xử lý được |

## 5.2 Result<T, E>

```rust
use std::fs;

fn main() {
    match fs::read_to_string("hello.txt") {
        Ok(noi_dung) => println!("{}", noi_dung),
        Err(loi) => println!("Lỗi: {}", loi),
    }
}
```

## 5.3 Xử Lý Result

```rust
fn main() {
    // unwrap - panic nếu Err
    let so: i32 = "42".parse().unwrap();

    // expect - panic với message
    let so: i32 = "42".parse().expect("Không phải số!");

    // unwrap_or - giá trị mặc định
    let so: i32 = "abc".parse().unwrap_or(0);
}
```

## 5.4 Toán Tử ?

```rust
use std::fs;
use std::io;

fn doc_file(path: &str) -> Result<String, io::Error> {
    let noi_dung = fs::read_to_string(path)?; // ? = return Err nếu lỗi
    Ok(noi_dung)
}
```

## 5.5 Custom Error

```rust
#[derive(Debug)]
enum AppError {
    IoError(std::io::Error),
    ParseError(String),
    ValidationError(String),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AppError::IoError(e) => write!(f, "IO: {}", e),
            AppError::ParseError(s) => write!(f, "Parse: {}", s),
            AppError::ValidationError(s) => write!(f, "Validation: {}", s),
        }
    }
}
```

## 5.6 Bài Tập

1. Hàm `chia_an_toan(a: f64, b: f64) -> Result<f64, String>`
2. Parse file cấu hình `key=value`
3. Validation struct `DangKy`

---

📖 **Trước đó**: [Chương 4](../chuong-04-struct-enum/README.md) | **Tiếp theo**: [Chương 6](../chuong-06-collections-generics/README.md)
