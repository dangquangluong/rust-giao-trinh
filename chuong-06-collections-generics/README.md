# Chương 6: Collections & Generics

## 6.1 Vector

```rust
fn main() {
    let mut so: Vec<i32> = vec![3, 1, 4, 1, 5];
    so.push(9);
    so.sort();
    println!("{:?}", so);

    let tong: i32 = so.iter().sum();
    let chan: Vec<&i32> = so.iter().filter(|&&x| x % 2 == 0).collect();
    println!("Tổng: {}, Chẵn: {:?}", tong, chan);
}
```

## 6.2 HashMap

```rust
use std::collections::HashMap;

fn main() {
    let mut diem: HashMap<&str, f64> = HashMap::new();
    diem.insert("Nguyễn A", 8.5);
    diem.insert("Trần B", 7.0);

    if let Some(d) = diem.get("Nguyễn A") {
        println!("Điểm: {}", d);
    }

    // Đếm từ
    let text = "rust rust go rust go";
    let mut dem: HashMap<&str, u32> = HashMap::new();
    for tu in text.split_whitespace() {
        *dem.entry(tu).or_insert(0) += 1;
    }
    println!("{:?}", dem);
}
```

## 6.3 Generics

```rust
fn tim_max<T: PartialOrd>(list: &[T]) -> &T {
    let mut max = &list[0];
    for item in &list[1..] {
        if item > max { max = item; }
    }
    max
}

struct Stack<T> { items: Vec<T> }

impl<T> Stack<T> {
    fn new() -> Self { Stack { items: Vec::new() } }
    fn push(&mut self, item: T) { self.items.push(item); }
    fn pop(&mut self) -> Option<T> { self.items.pop() }
}
```

## 6.4 Iterators

```rust
fn main() {
    let so = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let bp: Vec<i32> = so.iter().map(|&x| x * x).collect();
    let tong = so.iter().fold(0, |acc, &x| acc + x);
    let first_gt_5 = so.iter().find(|&&x| x > 5);

    println!("Bình phương: {:?}", bp);
    println!("Tổng: {}", tong);
    println!("Đầu tiên > 5: {:?}", first_gt_5);
}
```

## 6.5 Bài Tập

1. Quản lý sinh viên (Vec + HashMap)
2. Hàm generic tính trung bình
3. Iterator pipeline cho danh sách sản phẩm

---

📖 **Trước đó**: [Chương 5](../chuong-05-error-handling/README.md) | **Tiếp theo**: [Chương 7](../chuong-07-traits-lifetimes/README.md)
