# Bài Tập Chương 3: Ownership & Borrowing

## Bài 1: Sửa lỗi Move (⭐ Dễ)

**Yêu cầu:** Code sau bị lỗi. Sửa để nó chạy được (có 3 cách, thử cả 3):

```rust
fn main() {
    let s = String::from("hello");
    let s2 = s;
    println!("{}", s);   // ← LỖI ở đây!
}
```

**Gợi ý:**
- Cách 1: Dùng `.clone()`
- Cách 2: Dùng reference `&`
- Cách 3: Không tạo s2, dùng s trực tiếp

---

## Bài 2: Hàm dùng Reference (⭐⭐ Trung bình)

**Yêu cầu:** Viết hàm `dem_ky_tu(s: &str) -> usize` đếm số ký tự (characters, không phải bytes) trong chuỗi. Test với "Xin chào 🦀" (phải ra 9).

**Gợi ý:**
- Dùng `.chars().count()` thay vì `.len()` (len đếm bytes, chars đếm ký tự)
- Tham số `&str` = mượn chuỗi (không lấy ownership)

---

## Bài 3: Mutable Reference (⭐⭐ Trung bình)

**Yêu cầu:** Viết hàm `viet_hoa(s: &mut String)` biến toàn bộ chuỗi thành CHỮ HOA.

Test:
```
Input: "xin chào rust"
Output: "XIN CHÀO RUST"
```

**Gợi ý:**
- `s.make_ascii_uppercase()` chỉ hoạt động với ASCII
- Cho Unicode: `*s = s.to_uppercase()`

---

## Bài 4: Ownership trong hàm (⭐⭐⭐ Khó)

**Yêu cầu:** Viết hàm `noi_chuoi(s1: &str, s2: &str) -> String` nối 2 chuỗi lại. Sau khi gọi hàm, cả s1 và s2 vẫn dùng được.

Test:
```
s1 = "Xin", s2 = "chào"
Kết quả: "Xin chào"
Sau hàm: vẫn in được s1 và s2
```

**Gợi ý:**
- Dùng `format!("{} {}", s1, s2)` để tạo String mới
- Tham số `&str` = mượn, không lấy ownership
