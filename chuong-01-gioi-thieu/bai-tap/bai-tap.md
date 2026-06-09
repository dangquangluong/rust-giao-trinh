# Bài Tập Chương 1: Giới Thiệu Rust

## Bài 1: Hello World (⭐ Dễ)

**Yêu cầu:** Viết chương trình in ra tên và tuổi của bạn theo format:
```
Xin chào! Tôi là [tên], năm nay [tuổi] tuổi.
Tôi đang học Rust!
```

**Gợi ý:**
- Dùng `let` để khai báo biến
- Dùng `println!("... {}", bien)` để in ra

---

## Bài 2: Máy tính đơn giản (⭐ Dễ)

**Yêu cầu:** Khai báo 2 biến số `a = 10` và `b = 3`, in ra kết quả:
```
10 + 3 = 13
10 - 3 = 7
10 * 3 = 30
10 / 3 = 3
10 % 3 = 1
```

**Gợi ý:**
- Phép chia `/` cho số nguyên sẽ trả về số nguyên (bỏ phần thập phân)
- `%` là phép chia lấy dư (giống Python)

---

## Bài 3: Swap biến (⭐⭐ Trung bình)

**Yêu cầu:** Khai báo 2 biến `x = 5` và `y = 10`. Hoán đổi giá trị của chúng (không dùng biến tạm) rồi in ra.

**Gợi ý:**
- Rust có thể dùng tuple destructuring: `let (a, b) = (b, a);`
- Hoặc dùng shadowing

---

## Bài 4: Nhiệt độ (⭐⭐ Trung bình)

**Yêu cầu:** Viết hàm `celsius_sang_fahrenheit(c: f64) -> f64` chuyển đổi nhiệt độ. Công thức: `F = C * 9/5 + 32`.

In kết quả cho: 0°C, 25°C, 100°C.

**Gợi ý:**
- Hàm Rust: `fn ten_ham(tham_so: kieu) -> kieu_tra_ve { }`
- Dòng cuối không có `;` = return
