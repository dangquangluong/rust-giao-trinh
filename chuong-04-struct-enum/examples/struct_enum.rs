// === PYTHON EQUIVALENT ===
// # Python:
// # from dataclasses import dataclass
// #
// # @dataclass
// # class SinhVien:
// #     ten: str
// #     tuoi: int
// #     diem_tb: float
// #
// #     def hien_thi(self):
// #         print(f"📋 {self.ten} | Tuổi: {self.tuoi} | ĐTB: {self.diem_tb:.1f}")
// #
// # # Enum trong Python hiếm dùng, thường dùng class
// # import math
// # class HinhDang:
// #     def dien_tich(self): ...
// #
// # class HinhTron(HinhDang):
// #     def __init__(self, r): self.r = r
// #     def dien_tich(self): return math.pi * self.r ** 2

// ============================================
// Ví dụ Struct và Enum trong Rust
// Struct = giống @dataclass Python
// Enum = kiểu "chỉ có vài lựa chọn" (Python không có tương đương mạnh như vầy)
// ============================================

// --- STRUCT (giống @dataclass trong Python) ---
// struct = nơi chứa dữ liệu (giống class chỉ có __init__)
struct SinhVien {
    ten: String,     // String = chuỗi linh hoạt (giống str Python)
    tuoi: u8,        // u8 = số 0-255 (đủ cho tuổi)
    diem_tb: f64,    // f64 = số thực (giống float Python)
}

// impl = nơi viết methods (giống viết def trong class Python)
impl SinhVien {
    // fn moi() = "constructor" (giống __init__ Python)
    // &str = mượn chuỗi (không lấy ownership)
    // -> Self = trả về chính struct SinhVien
    fn moi(ten: &str, tuoi: u8, diem: f64) -> Self {
        SinhVien {
            ten: String::from(ten),  // Chuyển &str → String (giống str() Python)
            tuoi,                     // Viết tắt: tuoi: tuoi
            diem_tb: diem,
        }
    }

    // &self = giống self trong Python (mượn, không lấy ownership)
    fn hien_thi(&self) {
        // {:.1} = format 1 chữ số thập phân (giống :.1f Python)
        println!("📋 {} | Tuổi: {} | ĐTB: {:.1}", self.ten, self.tuoi, self.diem_tb);
    }
}

// --- ENUM (Python KHÔNG có thứ này!) ---
// Enum = "menu": chỉ có VÀI lựa chọn, mỗi lựa chọn chứa data khác nhau
// Giống Union type trong TypeScript, hoặc tagged union trong C
enum HinhDang {
    HinhTron(f64),                     // Chứa 1 số f64 (bán kính)
    HinhChuNhat { rong: f64, dai: f64 }, // Chứa 2 số (named fields)
    HinhVuong(f64),                    // Chứa 1 số f64 (cạnh)
}

impl HinhDang {
    // match = giống match/case Python 3.10, nhưng MẠNH hơn
    // Rust BẮT BUỘC bạn xử lý TẤT CẢ các trường hợp (exhaustive)
    fn dien_tich(&self) -> f64 {
        match self {
            // Mỗi nhánh: Pattern => kết quả
            HinhDang::HinhTron(r) => std::f64::consts::PI * r * r,  // PI * r²
            HinhDang::HinhChuNhat { rong, dai } => rong * dai,       // rộng * dài
            HinhDang::HinhVuong(a) => a * a,                         // cạnh²
        }
    }

    fn ten(&self) -> &str {
        match self {
            HinhDang::HinhTron(_) => "Hình tròn",      // _ = không cần giá trị bên trong
            HinhDang::HinhChuNhat { .. } => "HCN",     // .. = bỏ qua tất cả fields
            HinhDang::HinhVuong(_) => "Hình vuông",
        }
    }
}

fn main() {
    println!("=== STRUCT (giống @dataclass Python) ===\n");

    // vec![] = giống list [] Python, nhưng cùng kiểu
    let ds = vec![
        SinhVien::moi("Nguyễn A", 20, 8.5),  // :: = gọi "class method"
        SinhVien::moi("Trần B", 21, 6.0),
        SinhVien::moi("Lê C", 22, 9.2),
    ];

    // &ds = mượn vector (không move), giống for sv in ds: Python
    for sv in &ds {
        sv.hien_thi();  // Gọi method (giống sv.hien_thi() Python)
    }

    println!("\n=== ENUM (Python không có!) ===\n");

    let hinh = vec![
        HinhDang::HinhTron(5.0),                        // Tạo variant HinhTron
        HinhDang::HinhChuNhat { rong: 3.0, dai: 7.0 }, // Tạo variant HinhChuNhat
        HinhDang::HinhVuong(4.0),                       // Tạo variant HinhVuong
    ];

    for h in &hinh {
        // {:.2} = 2 chữ số thập phân (giống :.2f Python)
        println!("{}: S = {:.2}", h.ten(), h.dien_tich());
    }
}
