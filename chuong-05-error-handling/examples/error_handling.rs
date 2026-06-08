// Ví dụ xử lý lỗi
use std::num::ParseIntError;

#[derive(Debug)]
enum MayTinhError {
    ChiaChoKhong,
    SoKhongHopLe(String),
}

impl std::fmt::Display for MayTinhError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MayTinhError::ChiaChoKhong => write!(f, "Không thể chia cho 0!"),
            MayTinhError::SoKhongHopLe(s) => write!(f, "'{}' không hợp lệ", s),
        }
    }
}

fn chia(a: f64, b: f64) -> Result<f64, MayTinhError> {
    if b == 0.0 { Err(MayTinhError::ChiaChoKhong) }
    else { Ok(a / b) }
}

fn tinh(a_str: &str, b_str: &str) -> Result<f64, MayTinhError> {
    let a: f64 = a_str.parse().map_err(|_| MayTinhError::SoKhongHopLe(a_str.to_string()))?;
    let b: f64 = b_str.parse().map_err(|_| MayTinhError::SoKhongHopLe(b_str.to_string()))?;
    chia(a, b)
}

fn main() {
    let tests = vec![("10", "3"), ("10", "0"), ("abc", "5")];
    for (a, b) in tests {
        print!("{} / {} = ", a, b);
        match tinh(a, b) {
            Ok(kq) => println!("{:.2} ✅", kq),
            Err(e) => println!("LỖI: {} ❌", e),
        }
    }
}
