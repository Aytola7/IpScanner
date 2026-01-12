# 🚀 IP Scanner - Rust Implementation

پروژه IP Scanner با استفاده از Rust و Tokio بازنویسی شده است.

## 📁 ساختار پروژه

```
ip_scanner_rust/
├── Cargo.toml              # تنظیمات و dependencies پروژه
├── Cargo.lock              # نسخه دقیق dependencies
├── Makefile                # دستورات راحت برای build و run
├── README.md               # راهنمای کامل استفاده
├── COMPARISON.md           # مقایسه Python vs Rust
├── .gitignore              # فایل‌های ignore شده
├── ip4.txt                 # فایل IP های اصلی (کپی شده از پروژه Python)
├── example_ips.txt         # فایل مثال برای تست
└── src/
    ├── main.rs             # نقطه ورود برنامه + CLI
    ├── ip_creator.rs       # پارس و پردازش IP ها (CIDR, Range, Single)
    ├── worker_engine.rs    # مدیریت worker pool و message passing
    ├── task_handler.rs     # تست ping و socket برای IP ها
    └── timer.rs            # اندازه‌گیری زمان اجرا
```

## 🎯 ویژگی‌های پیاده‌سازی شده

✅ **Worker Pool Architecture**: 100 worker با 30 concurrency هر کدام (قابل تنظیم)
✅ **Async/Await**: استفاده از Tokio برای concurrency بهینه
✅ **IP Parsing**: پشتیبانی از CIDR, Range, و Single IP
✅ **Random Sampling**: امکان انتخاب درصدی از IP ها
✅ **Ping Test**: تست دسترسی با دستور ping
✅ **Socket Test**: تست پورت‌های رایج (13, 22, 23, 80, 443, 3389)
✅ **Logging**: سیستم log پیشرفته با env_logger
✅ **CLI Arguments**: پارامترهای قابل تنظیم با clap
✅ **Error Handling**: استفاده از Result و anyhow
✅ **Timer**: اندازه‌گیری دقیق زمان اجرا

## 🔧 نصب و اجرا

### پیش‌نیازها
```bash
# نصب Rust (اگر نصب نیست)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### کامپایل
```bash
cd ip_scanner_rust

# نسخه debug (برای توسعه)
cargo build

# نسخه release (بهینه شده)
cargo build --release
```

### اجرا

#### روش 1: با cargo
```bash
# با تنظیمات پیش‌فرض
cargo run --release

# با پارامترهای دلخواه
cargo run --release -- --workers 50 --concurrency 20

# با فایل IP دلخواه
cargo run --release -- --ip-file example_ips.txt
```

#### روش 2: با Makefile
```bash
# نمایش راهنما
make help

# اجرای نسخه release
make run-release

# تست با فایل مثال
make run-example

# اجرای سریع (worker کمتر)
make run-fast
```

#### روش 3: نصب سیستمی
```bash
cargo install --path .
# سپس می‌توانید مستقیماً اجرا کنید:
ip_scanner_rust --workers 100 --concurrency 30
```

## 📊 پارامترهای CLI

```bash
ip_scanner_rust [OPTIONS]

Options:
  -w, --workers <N>           تعداد workers (پیش‌فرض: 100)
  -c, --concurrency <N>       تعداد همزمانی در هر worker (پیش‌فرض: 30)
  -i, --ip-file <FILE>        مسیر فایل IP (پیش‌فرض: ip4.txt)
      --safe-ping <FILE>      فایل خروجی ping موفق (پیش‌فرض: safePing.txt)
      --safe-socket <FILE>    فایل خروجی socket موفق (پیش‌فرض: safeSocketConnect.txt)
  -h, --help                  نمایش راهنما
  -V, --version               نمایش نسخه
```

## 📝 فرمت فایل IP

```text
# CIDR notation
192.168.1.0/24
10.0.0.0/16

# IP Range
192.168.1.1-192.168.1.254
10.0.0.1-10.0.0.100

# Single IP
8.8.8.8
1.1.1.1
192.168.1.1
```

## 🎨 خروجی برنامه

```
[2026-01-13T00:15:30Z INFO  ip_scanner_rust] Starting IP Scanner
[2026-01-13T00:15:30Z INFO  ip_scanner_rust] Workers: 100, Concurrency per worker: 30
[2026-01-13T00:15:30Z INFO  ip_scanner_rust::worker_engine] [Worker-1] Started with concurrency 30
[2026-01-13T00:15:30Z INFO  ip_scanner_rust::worker_engine] [Worker-2] Started with concurrency 30
...
[2026-01-13T00:15:30Z INFO  ip_scanner_rust] Total IPs to scan: 245678
[2026-01-13T00:15:30Z INFO  ip_scanner_rust::task_handler] IP is available (ping): 8.8.8.8
[2026-01-13T00:15:31Z INFO  ip_scanner_rust::task_handler] IP is available (socket): 192.168.1.1:22
...
[2026-01-13T00:25:45Z INFO  ip_scanner_rust] Finished. Total Time: 0 hours, 10 minutes, 15 seconds
```

## 📂 فایل‌های خروجی

- `safePing.txt`: لیست IP هایی که به ping پاسخ دادند
- `safeSocketConnect.txt`: لیست IP:Port هایی که اتصال socket موفق بود

## 🔍 تست و Debug

```bash
# چک کردن بدون build
cargo check

# اجرا با log سطح debug
RUST_LOG=debug cargo run --release

# فرمت کردن کد
cargo fmt

# بررسی با clippy
cargo clippy

# تست‌ها
cargo test
```

## ⚡ Performance Tips

1. **Release Build**: همیشه با `--release` اجرا کنید (5-10x سریعتر)
2. **Worker Tuning**: تعداد worker = تعداد هسته‌های CPU × 2-4
3. **Concurrency**: برای شبکه، 20-50 مناسب است
4. **Memory**: هر worker حدود 0.5-1 MB RAM مصرف می‌کند

## 🐛 Troubleshooting

### مشکل: Permission denied برای ping
```bash
# راه‌حل 1: اجرا با sudo
sudo cargo run --release

# راه‌حل 2: دادن capability به binary
sudo setcap cap_net_raw+ep target/release/ip_scanner_rust
```

### مشکل: Too many open files
```bash
# افزایش محدودیت file descriptor
ulimit -n 10000
```

## 📦 Dependencies

- **tokio**: Async runtime
- **clap**: CLI argument parsing
- **log + env_logger**: Logging system
- **ipnetwork**: IP/CIDR utilities
- **rand**: Random sampling
- **chrono**: Date/time utilities
- **anyhow**: Error handling
- **async-trait**: Async traits
- **futures**: Async utilities

## 🎓 یادگیری بیشتر

- مقایسه با Python: [COMPARISON.md](COMPARISON.md)
- راهنمای کامل: [README.md](README.md)
- داکیومنت Rust: https://doc.rust-lang.org/
- Tokio Tutorial: https://tokio.rs/tokio/tutorial

## 📄 لایسنس

همان لایسنس پروژه اصلی Python

---

**نکته**: این نسخه Rust معادل کامل پروژه Python است با بهبود قابل توجه در سرعت و مصرف منابع.
