# Bài Tập Chương 2: Cú Pháp Cơ Bản

## Bài 1: Kiểm tra chẵn/lẻ (⭐ Dễ)

**Yêu cầu:** Viết hàm `la_chan(n: i32) -> bool` trả về true nếu n chẵn, false nếu lẻ. Test với các số: 4, 7, 0, -2.

**Gợi ý:**
- `n % 2 == 0` → chẵn
- Hàm trả bool: `-> bool`

---

## Bài 2: FizzBuzz (⭐ Dễ)

**Yêu cầu:** In số từ 1 đến 30:
- Chia hết cho 3: in "Fizz"
- Chia hết cho 5: in "Buzz"
- Chia hết cả 3 và 5: in "FizzBuzz"
- Còn lại: in số đó

**Gợi ý:**
- Dùng `for i in 1..=30`
- Kiểm tra `i % 15 == 0` trước (cả 3 và 5)

---

## Bài 3: Tính BMI (⭐⭐ Trung bình)

**Yêu cầu:** Viết hàm `tinh_bmi(can_nang_kg: f64, chieu_cao_m: f64) -> f64` và hàm `xep_loai_bmi(bmi: f64) -> &'static str` trả về:
- < 18.5: "Gầy"
- 18.5-24.9: "Bình thường"
- 25-29.9: "Thừa cân"
- >= 30: "Béo phì"

**Gợi ý:**
- BMI = cân_nặng / (chiều_cao * chiều_cao)
- Dùng if/else if/else

---

## Bài 4: Bảng cửu chương (⭐⭐ Trung bình)

**Yêu cầu:** In bảng cửu chương của số n (user chọn từ 2-9). Format:
```
=== Bảng cửu chương 5 ===
5 x 1 = 5
5 x 2 = 10
...
5 x 10 = 50
```

**Gợi ý:**
- Dùng `for i in 1..=10`
- Dùng `println!("{} x {} = {}", n, i, n * i)`

---

## Bài 5: Đếm ngược (⭐⭐ Trung bình)

**Yêu cầu:** Viết hàm `dem_nguoc(n: u32)` in ra n, n-1, ..., 2, 1, "Phóng! 🚀". Dùng cả 3 cách: `for`, `while`, `loop`.

**Gợi ý:**
- `for`: dùng `(1..=n).rev()` để đảo ngược range
- `while`: giảm biến đếm
- `loop`: dùng `break` khi đến 0
