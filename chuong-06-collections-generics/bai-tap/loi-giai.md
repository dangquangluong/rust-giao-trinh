# Lời Giải Chương 6

## Bài 1: Đếm từ

```rust
use std::collections::HashMap;

fn dem_tu(text: &str) -> HashMap<&str, u32> {
    let mut map = HashMap::new();                // Tạo HashMap rỗng
    for tu in text.split_whitespace() {          // Tách theo khoảng trắng
        *map.entry(tu).or_insert(0) += 1;        // Tăng đếm (tạo mới nếu chưa có)
    }
    map                                          // Return HashMap
}

fn main() {
    let text = "rust là tuyệt rust rất nhanh rust rất an toàn";
    let kq = dem_tu(text);
    for (tu, dem) in &kq {
        println!("  '{}': {} lần", tu, dem);
    }
}
```

## Bài 2: Thống kê

```rust
fn main() {
    let diem = vec![8.5, 3.0, 7.0, 9.2, 4.5, 6.0, 5.5];

    let tong: f64 = diem.iter().sum();                      // Tổng
    let tb = tong / diem.len() as f64;                      // Trung bình
    let dat: usize = diem.iter().filter(|&&d| d >= 5.0).count();  // Đếm >= 5
    let max = diem.iter().cloned().fold(f64::MIN, f64::max);      // Max

    println!("TB: {:.2}, Đạt: {}/{}, Max: {}", tb, dat, diem.len(), max);
}
```

## Bài 3: Generic Stack

```rust
struct Stack<T> {                        // Generic struct
    items: Vec<T>,                       // Vec bên trong chứa data
}

impl<T> Stack<T> {                       // impl cho mọi kiểu T
    fn new() -> Self {                   // Constructor
        Stack { items: Vec::new() }
    }
    fn push(&mut self, item: T) {        // Thêm vào đỉnh
        self.items.push(item);
    }
    fn pop(&mut self) -> Option<T> {     // Lấy ra từ đỉnh (có thể rỗng)
        self.items.pop()
    }
    fn peek(&self) -> Option<&T> {       // Xem đỉnh không lấy ra
        self.items.last()
    }
    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
    fn len(&self) -> usize {
        self.items.len()
    }
}

fn main() {
    let mut stack: Stack<i32> = Stack::new();
    stack.push(1); stack.push(2); stack.push(3);
    println!("Peek: {:?}", stack.peek());        // Some(3)
    println!("Pop: {:?}", stack.pop());          // Some(3)
    println!("Len: {}", stack.len());            // 2
}
```
