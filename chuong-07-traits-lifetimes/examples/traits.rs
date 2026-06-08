// === PYTHON EQUIVALENT ===
// # Python:
// # from abc import ABC, abstractmethod
// #
// # class HinhHoc(ABC):
// #     @abstractmethod
// #     def dien_tich(self) -> float: ...
// #     @abstractmethod
// #     def ten(self) -> str: ...
// #
// # class HinhTron(HinhHoc):
// #     def __init__(self, r): self.r = r
// #     def dien_tich(self): return 3.14159 * self.r ** 2
// #     def ten(self): return "Hình tròn"
// #
// # class HinhVuong(HinhHoc):
// #     def __init__(self, a): self.a = a
// #     def dien_tich(self): return self.a ** 2
// #     def ten(self): return "Hình vuông"
// #
// # # Duck typing: bất kỳ object nào có .dien_tich() đều dùng được
// # shapes = [HinhTron(5), HinhVuong(4)]
// # for s in shapes:
// #     print(f"{s.ten()}: S = {s.dien_tich():.2f}")

// ============================================
// Traits trong Rust = ABC/Protocol trong Python
// Trait = "hợp đồng" quy định struct phải có method gì
// ============================================

// --- TRAIT (giống ABC Python) ---
// trait = khai báo "interface": muốn là HinhHoc thì PHẢI có dien_tich() và ten()
trait HinhHoc {
    fn dien_tich(&self) -> f64;   // Bắt buộc implement (giống @abstractmethod)
    fn ten(&self) -> &str;        // &str = trả reference chuỗi
}

// --- STRUCT (giống class chỉ có data) ---
struct HinhTron { ban_kinh: f64 }  // Chỉ chứa data, không có method ở đây
struct HinhVuong { canh: f64 }

// --- IMPL TRAIT (giống class HinhTron(HinhHoc): trong Python) ---
impl HinhHoc for HinhTron {
    // Implement method bắt buộc
    // .powi(2) = lũy thừa nguyên (giống ** 2 Python)
    fn dien_tich(&self) -> f64 { std::f64::consts::PI * self.ban_kinh.powi(2) }
    fn ten(&self) -> &str { "Hình tròn" }
}

impl HinhHoc for HinhVuong {
    fn dien_tich(&self) -> f64 { self.canh * self.canh }
    fn ten(&self) -> &str { "Hình vuông" }
}

// --- TRAIT OBJECT (giống duck typing Python) ---
// &dyn HinhHoc = "bất kỳ kiểu nào implement HinhHoc" (giống ABC type hint)
// Giống Python: def in_info(h: HinhHoc): ...
fn in_info(h: &dyn HinhHoc) {
    println!("{}: S = {:.2}", h.ten(), h.dien_tich());
}

// --- LIFETIME (khái niệm CHỈ RUST CÓ) ---
// <'a> = "lifetime parameter" = nói cho compiler biết reference sống bao lâu
// Đọc: "hàm longest trả reference sống ít nhất bằng x VÀ y"
// Bạn KHÔNG CẦN hiểu sâu ngay. Chỉ cần biết:
// - Khi hàm trả về reference từ 2+ tham số → cần ghi 'a
// - Hầu hết trường hợp khác Rust tự suy luận
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    // --- Trait Object: giống list chứa nhiều kiểu khác nhau ---
    // Python: shapes = [HinhTron(5), HinhVuong(4)]  # dễ, duck typing
    // Rust: phải dùng Box<dyn Trait> vì mỗi kiểu có size khác nhau
    // Box = "đóng hộp" để đặt trên heap (đừng lo, cứ dùng pattern này)
    let shapes: Vec<Box<dyn HinhHoc>> = vec![
        Box::new(HinhTron { ban_kinh: 5.0 }),
        Box::new(HinhVuong { canh: 4.0 }),
    ];

    // .as_ref() = lấy reference từ Box (giống lấy object ra khỏi hộp)
    for s in &shapes {
        in_info(s.as_ref());
    }
    // Output:
    // Hình tròn: S = 78.54
    // Hình vuông: S = 16.00

    // --- Lifetime example ---
    let s1 = "hello world";  // &str = string literal (sống mãi)
    let s2 = "hi";
    // longest chọn chuỗi dài hơn và trả reference
    println!("\nDài hơn: '{}'", longest(s1, s2));
    // Output: Dài hơn: 'hello world'
}
