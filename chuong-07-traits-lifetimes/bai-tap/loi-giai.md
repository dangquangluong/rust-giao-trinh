# Lời Giải Chương 7

## Bài 1: Trait Printable

```rust
trait Printable {
    fn to_string(&self) -> String;       // Method bắt buộc
}

struct SinhVien { ten: String, diem: f64 }
struct SanPham { ten: String, gia: f64 }

impl Printable for SinhVien {
    fn to_string(&self) -> String {
        format!("SV: {} (điểm: {})", self.ten, self.diem)
    }
}

impl Printable for SanPham {
    fn to_string(&self) -> String {
        format!("SP: {} (giá: {}đ)", self.ten, self.gia)
    }
}

fn in_ra(item: &dyn Printable) {         // dyn = dynamic dispatch
    println!("{}", item.to_string());
}

fn main() {
    let sv = SinhVien { ten: String::from("An"), diem: 8.5 };
    let sp = SanPham { ten: String::from("Laptop"), gia: 15_000_000.0 };
    in_ra(&sv);
    in_ra(&sp);
}
```

## Bài 2: Trait Object - Tổng diện tích

```rust
trait HinhHoc {
    fn dien_tich(&self) -> f64;
    fn ten(&self) -> &str;
}

struct HinhTron { r: f64 }
struct HinhVuong { a: f64 }

impl HinhHoc for HinhTron {
    fn dien_tich(&self) -> f64 { 3.14159 * self.r * self.r }
    fn ten(&self) -> &str { "Hình tròn" }
}
impl HinhHoc for HinhVuong {
    fn dien_tich(&self) -> f64 { self.a * self.a }
    fn ten(&self) -> &str { "Hình vuông" }
}

fn main() {
    let shapes: Vec<Box<dyn HinhHoc>> = vec![
        Box::new(HinhTron { r: 5.0 }),
        Box::new(HinhVuong { a: 4.0 }),
        Box::new(HinhTron { r: 3.0 }),
    ];
    let tong: f64 = shapes.iter().map(|s| s.dien_tich()).sum();
    println!("Tổng diện tích: {:.2}", tong);     // 78.54 + 16 + 28.27 = 122.81
}
```

## Bài 3: Generic hàm

```rust
fn in_list<T: std::fmt::Display>(items: &[T]) {  // T phải Display được
    for (i, item) in items.iter().enumerate() {    // enumerate = đánh số
        println!("  {}. {}", i + 1, item);
    }
}

fn main() {
    let so = vec![10, 20, 30];
    let ten = vec!["An", "Bình", "Cường"];
    println!("Số:"); in_list(&so);
    println!("Tên:"); in_list(&ten);
}
```
