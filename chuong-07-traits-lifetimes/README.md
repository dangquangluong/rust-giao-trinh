# Chương 7: Traits & Lifetimes

## 7.1 Traits

```rust
trait HinhHoc {
    fn dien_tich(&self) -> f64;
    fn chu_vi(&self) -> f64;
    fn mo_ta(&self) -> String {
        format!("S={:.2}, C={:.2}", self.dien_tich(), self.chu_vi())
    }
}

struct HinhTron { ban_kinh: f64 }
impl HinhHoc for HinhTron {
    fn dien_tich(&self) -> f64 { std::f64::consts::PI * self.ban_kinh.powi(2) }
    fn chu_vi(&self) -> f64 { 2.0 * std::f64::consts::PI * self.ban_kinh }
}
```

## 7.2 Trait Bound

```rust
fn in_info(h: &impl HinhHoc) { println!("{}", h.mo_ta()); }
fn in_info2<T: HinhHoc>(h: &T) { println!("{}", h.mo_ta()); }
```

## 7.3 Derive Traits

```rust
#[derive(Debug, Clone, PartialEq)]
struct Point { x: f64, y: f64 }
```

| Trait | Dùng cho |
|-------|----------|
| `Debug` | `{:?}` |
| `Clone` | `.clone()` |
| `PartialEq` | `==` |
| `Default` | giá trị mặc định |

## 7.4 Trait Objects

```rust
let shapes: Vec<Box<dyn HinhHoc>> = vec![
    Box::new(HinhTron { ban_kinh: 5.0 }),
    // ...
];
```

## 7.5 Lifetimes

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

## 7.6 Bài Tập

1. Trait `Serializable` với `to_json()`
2. Generic `Stack<T>`
3. Lifetime: `split_at_word`

---

📖 **Trước đó**: [Chương 6](../chuong-06-collections-generics/README.md) | **Tiếp theo**: [Chương 8](../chuong-08-concurrency/README.md)
