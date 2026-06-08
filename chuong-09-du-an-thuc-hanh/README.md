# Chương 9: Dự Án Thực Hành

## Dự Án 1: Todo CLI App ⭐⭐

Ứng dụng quản lý công việc chạy trên terminal.

```bash
cargo run -- them "Học Rust"
cargo run -- list
cargo run -- xong 1
cargo run -- xoa 1
```

**Kiến thức**: Struct, Vec, File I/O, serde_json, Error handling

## Dự Án 2: Grep Clone ⭐⭐

Tìm kiếm text trong file/thư mục (như grep).

```bash
cargo run -- -n "pattern" ./src
```

**Kiến thức**: File I/O, Recursion, CLI args

## Dự Án 3: HTTP Client ⭐⭐⭐

CLI tool gửi HTTP requests.

```bash
cargo run -- get https://api.example.com
cargo run -- post https://api.example.com --body '{"key":"value"}'
```

**Kiến thức**: reqwest, clap, async, Error handling

## Dự Án Nâng Cao

| # | Dự Án | Kiến Thức |
|---|--------|-----------|
| 4 | Chat server TCP | Threads, Arc/Mutex |
| 5 | Markdown → HTML | String processing |
| 6 | JSON Parser | Enum, Recursion |
| 7 | Web API (Axum) | Async, Web framework |

## 🎓 Tài Nguyên Tiếp Theo

- [Rust Book](https://doc.rust-lang.org/book/)
- [Exercism Rust Track](https://exercism.org/tracks/rust)
- [Advent of Code](https://adventofcode.com/)

---

🎉 **Chúc mừng hoàn thành giáo trình!**

📖 **Trước đó**: [Chương 8](../chuong-08-concurrency/README.md)
