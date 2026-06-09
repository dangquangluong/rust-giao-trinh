# Lời Giải Chương 4

## Bài 1: Hình chữ nhật

```rust
struct HinhChuNhat {                     // Struct chứa data
    rong: f64,                           // Chiều rộng
    dai: f64,                            // Chiều dài
}

impl HinhChuNhat {                       // Thêm methods
    fn dien_tich(&self) -> f64 {         // &self = mượn struct
        self.rong * self.dai             // rộng × dài
    }

    fn chu_vi(&self) -> f64 {
        2.0 * (self.rong + self.dai)     // 2 × (rộng + dài)
    }

    fn la_hinh_vuong(&self) -> bool {
        self.rong == self.dai            // Vuông khi rộng = dài
    }
}

fn main() {
    let hcn = HinhChuNhat { rong: 5.0, dai: 10.0 };
    println!("Diện tích: {}", hcn.dien_tich());       // 50
    println!("Chu vi: {}", hcn.chu_vi());             // 30
    println!("Là hình vuông: {}", hcn.la_hinh_vuong()); // false
}
```

## Bài 2: Enum phương tiện

```rust
enum PhuongTien {
    XeDap,                               // Không chứa data
    XeMay(String),                       // Chứa tên hãng
    OTo { hang: String, so_cho: u8 },    // Chứa nhiều fields
}

fn mo_ta(pt: &PhuongTien) {              // &PhuongTien = mượn enum
    match pt {                            // match = xét từng variant
        PhuongTien::XeDap => println!("🚲 Xe đạp"),
        PhuongTien::XeMay(hang) => println!("🏍️ Xe máy {}", hang),
        PhuongTien::OTo { hang, so_cho } => {
            println!("🚗 Ô tô {} ({} chỗ)", hang, so_cho);
        }
    }
}

fn main() {
    let ds = vec![
        PhuongTien::XeDap,
        PhuongTien::XeMay(String::from("Honda")),
        PhuongTien::OTo { hang: String::from("Toyota"), so_cho: 5 },
    ];
    for pt in &ds { mo_ta(pt); }
}
```

## Bài 3: Option - Chia an toàn

```rust
fn chia(a: f64, b: f64) -> Option<f64> { // Option = có thể None
    if b == 0.0 {
        None                              // Không chia được → None
    } else {
        Some(a / b)                       // Có kết quả → Some(giá_trị)
    }
}

fn main() {
    let tests = [(10.0, 3.0), (10.0, 0.0), (7.5, 2.5)];
    for (a, b) in tests {
        match chia(a, b) {                // Xử lý Option
            Some(kq) => println!("{}/{} = {:.2}", a, b, kq),
            None => println!("{}/{} = Lỗi chia cho 0!", a, b),
        }
    }
}
```

## Bài 4: Danh bạ

```rust
struct LienHe {
    ten: String,
    sdt: String,
    email: Option<String>,               // Có thể không có email
}

fn tim_kiem<'a>(ds: &'a [LienHe], ten: &str) -> Option<&'a LienHe> {
    ds.iter().find(|lh| lh.ten == ten)   // .find() trả Option
}

fn main() {
    let ds = vec![
        LienHe { ten: String::from("An"), sdt: String::from("0901234567"), email: Some(String::from("an@mail.com")) },
        LienHe { ten: String::from("Binh"), sdt: String::from("0912345678"), email: None },
    ];

    match tim_kiem(&ds, "An") {
        Some(lh) => {
            println!("Tìm thấy: {} - {}", lh.ten, lh.sdt);
            match &lh.email {
                Some(e) => println!("  Email: {}", e),
                None => println!("  Không có email"),
            }
        }
        None => println!("Không tìm thấy!"),
    }
}
```
