// Ví dụ Struct và Enum
struct SinhVien {
    ten: String,
    tuoi: u8,
    diem_tb: f64,
}

impl SinhVien {
    fn moi(ten: &str, tuoi: u8, diem: f64) -> Self {
        SinhVien { ten: String::from(ten), tuoi, diem_tb: diem }
    }
    fn hien_thi(&self) {
        println!("📋 {} | Tuổi: {} | ĐTB: {:.1}", self.ten, self.tuoi, self.diem_tb);
    }
}

enum HinhDang {
    HinhTron(f64),
    HinhChuNhat { rong: f64, dai: f64 },
    HinhVuong(f64),
}

impl HinhDang {
    fn dien_tich(&self) -> f64 {
        match self {
            HinhDang::HinhTron(r) => std::f64::consts::PI * r * r,
            HinhDang::HinhChuNhat { rong, dai } => rong * dai,
            HinhDang::HinhVuong(a) => a * a,
        }
    }
    fn ten(&self) -> &str {
        match self {
            HinhDang::HinhTron(_) => "Hình tròn",
            HinhDang::HinhChuNhat { .. } => "HCN",
            HinhDang::HinhVuong(_) => "Hình vuông",
        }
    }
}

fn main() {
    println!("=== STRUCT ===\n");
    let ds = vec![
        SinhVien::moi("Nguyễn A", 20, 8.5),
        SinhVien::moi("Trần B", 21, 6.0),
        SinhVien::moi("Lê C", 22, 9.2),
    ];
    for sv in &ds { sv.hien_thi(); }

    println!("\n=== ENUM ===\n");
    let hinh = vec![
        HinhDang::HinhTron(5.0),
        HinhDang::HinhChuNhat { rong: 3.0, dai: 7.0 },
        HinhDang::HinhVuong(4.0),
    ];
    for h in &hinh {
        println!("{}: S = {:.2}", h.ten(), h.dien_tich());
    }
}
