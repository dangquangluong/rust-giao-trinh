# Lời Giải Chương 2

## Bài 1: Kiểm tra chẵn/lẻ

```rust
fn la_chan(n: i32) -> bool {            // Hàm trả về bool
    n % 2 == 0                           // true nếu chia hết cho 2, không ; = return
}

fn main() {
    let test = [4, 7, 0, -2];           // Array test
    for &n in &test {                    // &test = mượn, &n = destructure reference
        println!("{} là {}", n, if la_chan(n) { "chẵn" } else { "lẻ" });
    }
}
```

---

## Bài 2: FizzBuzz

```rust
fn main() {
    for i in 1..=30 {                    // 1 đến 30 (bao gồm 30)
        if i % 15 == 0 {                 // Chia hết 3 VÀ 5 (kiểm tra trước!)
            println!("FizzBuzz");
        } else if i % 3 == 0 {          // Chia hết 3
            println!("Fizz");
        } else if i % 5 == 0 {          // Chia hết 5
            println!("Buzz");
        } else {                          // Còn lại
            println!("{}", i);
        }
    }
}
```

---

## Bài 3: Tính BMI

```rust
fn tinh_bmi(can_nang_kg: f64, chieu_cao_m: f64) -> f64 {
    can_nang_kg / (chieu_cao_m * chieu_cao_m)   // BMI = w / h²
}

fn xep_loai_bmi(bmi: f64) -> &'static str {     // &'static str = chuỗi cố định
    if bmi < 18.5 {
        "Gầy"
    } else if bmi < 25.0 {
        "Bình thường"
    } else if bmi < 30.0 {
        "Thừa cân"
    } else {
        "Béo phì"
    }
}

fn main() {
    let bmi = tinh_bmi(70.0, 1.75);              // 70kg, 1.75m
    println!("BMI: {:.1} - {}", bmi, xep_loai_bmi(bmi));
    // Output: BMI: 22.9 - Bình thường
}
```

---

## Bài 4: Bảng cửu chương

```rust
fn main() {
    let n = 5;                           // Bảng cửu chương 5
    println!("=== Bảng cửu chương {} ===", n);
    for i in 1..=10 {                    // 1 đến 10
        println!("{} x {} = {}", n, i, n * i);
    }
}
```

---

## Bài 5: Đếm ngược (3 cách)

```rust
fn dem_nguoc_for(n: u32) {
    println!("--- Cách 1: for ---");
    for i in (1..=n).rev() {             // .rev() = đảo ngược range
        println!("{}...", i);
    }
    println!("Phóng! 🚀");
}

fn dem_nguoc_while(n: u32) {
    println!("--- Cách 2: while ---");
    let mut i = n;                       // mut vì sẽ giảm
    while i > 0 {                        // Lặp khi i > 0
        println!("{}...", i);
        i -= 1;                          // Giảm i
    }
    println!("Phóng! 🚀");
}

fn dem_nguoc_loop(n: u32) {
    println!("--- Cách 3: loop ---");
    let mut i = n;
    loop {                               // Lặp vô hạn
        if i == 0 {                      // Điều kiện dừng
            break;                       // Thoát loop
        }
        println!("{}...", i);
        i -= 1;
    }
    println!("Phóng! 🚀");
}

fn main() {
    dem_nguoc_for(5);
    dem_nguoc_while(5);
    dem_nguoc_loop(5);
}
```
