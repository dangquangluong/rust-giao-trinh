# Bài Tập Chương 5: Error Handling

## Bài 1: Chia an toàn với Result (⭐ Dễ)

**Yêu cầu:** Viết `chia(a: f64, b: f64) -> Result<f64, String>` trả Err khi b=0.

---

## Bài 2: Parse config (⭐⭐ Trung bình)

**Yêu cầu:** Viết hàm `parse_config(line: &str) -> Result<(String, String), String>` parse chuỗi dạng "key=value". Trả Err nếu không có dấu "=".

---

## Bài 3: Chuỗi ? operator (⭐⭐⭐ Khó)

**Yêu cầu:** Viết hàm `doc_va_tinh(path: &str) -> Result<f64, Box<dyn std::error::Error>>` đọc file chứa các số (mỗi dòng 1 số), tính trung bình. Dùng `?` cho mỗi bước có thể lỗi.
