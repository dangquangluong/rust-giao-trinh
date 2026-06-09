# Bài Tập Chương 8: Concurrency

## Bài 1: Map-Reduce (⭐⭐ Trung bình)

**Yêu cầu:** Chia Vec<i32> thành 4 phần, tạo 4 threads tính tổng mỗi phần, cuối cùng gộp lại.

---

## Bài 2: Producer-Consumer (⭐⭐⭐ Khó)

**Yêu cầu:** 3 producer threads gửi số qua channel, 1 consumer (thread chính) nhận và tính tổng.

---

## Bài 3: Shared Counter (⭐⭐ Trung bình)

**Yêu cầu:** Dùng Arc<Mutex<Vec<String>>>, 5 threads mỗi thread thêm tên mình vào Vec. In kết quả cuối.
