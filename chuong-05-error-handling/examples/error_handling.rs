// === PYTHON EQUIVALENT ===
// # Python:
// # class MayTinhError(Exception):
// #     pass
// #
// # def chia(a, b):
// #     if b == 0:
// #         raise MayTinhError("Không thể chia cho 0!")
// #     return a / b
// #
// # def tinh(a_str, b_str):
// #     try:
// #         a = float(a_str)
// #         b = float(b_str)
// #         return chia(a, b)
// #     except ValueError:
// #         raise MayTinhError(f"'{a_str}' hoặc '{b_str}' không hợp lệ")
// #
// # tests = [("10", "3"), ("10", "0"), ("abc", "5")]
// # for a, b in tests:
// #     try:
// #         print(f"{a}/{b} = {tinh(a, b):.2f} ✅")
// #     except MayTinhError as e:
// #         print(f"{a}/{b} = LỖI: {e} ❌")

// ============================================
// Error Handling trong Rust
// Rust KHÔNG có try/except!
// Thay vào đó dùng: Result<T, E> = Ok(giá_trị) hoặc Err(lỗi)
// ============================================

// use = giống import Python
use std::num::ParseIntError;

// --- TẠO CUSTOM ERROR (giống class MyError(Exception): trong Python) ---
// #[derive(Debug)] = tự động thêm khả năng in debug (giống __repr__)
#[derive(Debug)]
enum MayTinhError {
    ChiaChoKhong,              // Variant 1: chia cho 0
    SoKhongHopLe(String),     // Variant 2: chứa message lỗi
}

// impl Display = giống __str__ trong Python (quyết định print ra gì)
impl std::fmt::Display for MayTinhError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MayTinhError::ChiaChoKhong => write!(f, "Không thể chia cho 0!"),
            MayTinhError::SoKhongHopLe(s) => write!(f, "'{}' không hợp lệ", s),
        }
    }
}

// --- HÀM TRẢ VỀ RESULT (thay vì raise exception) ---
// Result<f64, MayTinhError> = "trả về f64 nếu OK, MayTinhError nếu lỗi"
// Giống Python: def chia(a, b) -> float:  (nhưng có thể raise)
fn chia(a: f64, b: f64) -> Result<f64, MayTinhError> {
    if b == 0.0 {
        Err(MayTinhError::ChiaChoKhong)  // Err() = giống raise
    } else {
        Ok(a / b)  // Ok() = giống return bình thường
    }
}

// --- TOÁN TỬ ? = "nếu lỗi thì return lỗi luôn" ---
// Giống Python: không có equivalent trực tiếp
// Gần nhất: a = float(a_str)  # nhưng tự raise nếu lỗi
fn tinh(a_str: &str, b_str: &str) -> Result<f64, MayTinhError> {
    // .parse() = giống float() Python, có thể lỗi
    // .map_err(|_| ...) = "nếu lỗi, đổi thành error của mình"
    // ? = "nếu Err → return Err ngay" (giống raise lại)
    let a: f64 = a_str.parse()
        .map_err(|_| MayTinhError::SoKhongHopLe(a_str.to_string()))?;

    let b: f64 = b_str.parse()
        .map_err(|_| MayTinhError::SoKhongHopLe(b_str.to_string()))?;

    chia(a, b)  // Gọi hàm chia, tự trả Result
}

fn main() {
    // vec![] = giống list Python
    let tests = vec![("10", "3"), ("10", "0"), ("abc", "5")];

    for (a, b) in tests {
        print!("{} / {} = ", a, b);

        // match Result = giống try/except Python
        // Ok(kq) = không lỗi, lấy kết quả
        // Err(e) = có lỗi, lấy error
        match tinh(a, b) {
            Ok(kq) => println!("{:.2} ✅", kq),   // {:.2} = 2 số thập phân
            Err(e) => println!("LỖI: {} ❌", e),  // In error message
        }
    }

    // Kết quả:
    // 10 / 3 = 3.33 ✅
    // 10 / 0 = LỖI: Không thể chia cho 0! ❌
    // abc / 5 = LỖI: 'abc' không hợp lệ ❌
}
