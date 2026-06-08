# Chương 4: Struct, Enum & Pattern Matching

## 4.1 Struct

```rust
struct SinhVien {
    ten: String,
    tuoi: u8,
    diem_tb: f64,
}

impl SinhVien {
    fn moi(ten: &str, tuoi: u8, diem: f64) -> Self {
        SinhVien { ten: String::from(ten), tuoi, diem_tb: diem }
    }

    fn xep_loai(&self) -> &str {
        if self.diem_tb >= 8.0 { "Giỏi" }
        else if self.diem_tb >= 6.5 { "Khá" }
        else { "Trung bình" }
    }
}
```

## 4.2 Enum

```rust
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
}
```

## 4.3 Option<T>

Rust không có null! Dùng `Option<T>`:

```rust
fn chia(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 { None } else { Some(a / b) }
}

fn main() {
    match chia(10.0, 3.0) {
        Some(kq) => println!("Kết quả: {:.2}", kq),
        None => println!("Không thể chia cho 0!"),
    }
}
```

## 4.4 Pattern Matching

```rust
fn main() {
    let so = 13;
    match so {
        1..=12 => println!("Từ 1-12"),
        13 => println!("Số 13!"),
        _ => println!("Số khác"),
    }

    // if let - khi chỉ quan tâm 1 pattern
    let val: Option<i32> = Some(42);
    if let Some(x) = val {
        println!("Giá trị: {}", x);
    }
}
```

## 4.5 Bài Tập

1. Tạo struct `Sach` với methods
2. Tạo enum `PhuongTien` với thông tin
3. Enum `PhepTinh` cho máy tính

---

📖 **Trước đó**: [Chương 3](../chuong-03-ownership-borrowing/README.md) | **Tiếp theo**: [Chương 5](../chuong-05-error-handling/README.md)
