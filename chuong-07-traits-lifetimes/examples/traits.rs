trait HinhHoc {
    fn dien_tich(&self) -> f64;
    fn ten(&self) -> &str;
}

struct HinhTron { ban_kinh: f64 }
struct HinhVuong { canh: f64 }

impl HinhHoc for HinhTron {
    fn dien_tich(&self) -> f64 { std::f64::consts::PI * self.ban_kinh.powi(2) }
    fn ten(&self) -> &str { "Hình tròn" }
}

impl HinhHoc for HinhVuong {
    fn dien_tich(&self) -> f64 { self.canh * self.canh }
    fn ten(&self) -> &str { "Hình vuông" }
}

// Generic function
fn in_info(h: &dyn HinhHoc) {
    println!("{}: S = {:.2}", h.ten(), h.dien_tich());
}

// Lifetime example
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let shapes: Vec<Box<dyn HinhHoc>> = vec![
        Box::new(HinhTron { ban_kinh: 5.0 }),
        Box::new(HinhVuong { canh: 4.0 }),
    ];

    for s in &shapes { in_info(s.as_ref()); }

    let s1 = "hello world";
    let s2 = "hi";
    println!("\nDài hơn: '{}'", longest(s1, s2));
}
