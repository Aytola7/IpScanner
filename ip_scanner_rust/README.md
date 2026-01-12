# IP Scanner - Rust Version

این نسخه Rust از پروژه IP Scanner است که با استفاده از async/await و Tokio پیاده‌سازی شده.

## ویژگی‌ها

- ✅ اسکن همزمان هزاران IP با استفاده از معماری worker pool
- ✅ تست IP ها با ping و socket connection
- ✅ پشتیبانی از CIDR notation (مثل 192.168.1.0/24)
- ✅ پشتیبانی از IP range (مثل 192.168.1.1-192.168.1.254)
- ✅ نمونه‌گیری تصادفی از IP ها
- ✅ ذخیره نتایج در فایل
- ✅ Logging پیشرفته
- ✅ تایمر برای اندازه‌گیری زمان اجرا

## نصب و راه‌اندازی

### پیش‌نیازها

- Rust 1.70 یا بالاتر
- دسترسی به دستور `ping` در سیستم

### کامپایل

```bash
cd ip_scanner_rust
cargo build --release
```

### اجرا

```bash
# استفاده با تنظیمات پیش‌فرض (100 worker، 30 concurrency)
cargo run --release

# تنظیم تعداد worker و concurrency
cargo run --release -- --workers 50 --concurrency 20

# مشخص کردن فایل IP
cargo run --release -- --ip-file my_ips.txt

# تمام گزینه‌ها
cargo run --release -- \
    --workers 100 \
    --concurrency 30 \
    --ip-file ip4.txt \
    --safe-ping safePing.txt \
    --safe-socket safeSocketConnect.txt
```

### نمایش راهنما

```bash
cargo run --release -- --help
```

## فرمت فایل IP

فایل IP می‌تواند شامل این فرمت‌ها باشد:

```
# CIDR notation
192.168.1.0/24
10.0.0.0/16

# IP Range
192.168.1.1-192.168.1.254

# تک IP
192.168.1.1
8.8.8.8
```

## خروجی

- `safePing.txt`: لیست IP هایی که به ping پاسخ دادند
- `safeSocketConnect.txt`: لیست IP:Port هایی که socket connection موفق بود

## مقایسه با نسخه Python

| ویژگی | Python | Rust |
|-------|--------|------|
| سرعت | ⭐⭐⭐ | ⭐⭐⭐⭐⭐ |
| مصرف حافظه | متوسط | خیلی کم |
| همزمانی | asyncio + threading | Tokio (pure async) |
| Type Safety | Dynamic | Static + Strong |
| Error Handling | Exceptions | Result<T, E> |

## معماری

```
main.rs
├── ip_creator.rs      # خواندن و پردازش فایل IP
├── worker_engine.rs   # مدیریت worker pool و توزیع تسک‌ها
├── task_handler.rs    # اجرای تست‌های ping و socket
└── timer.rs          # اندازه‌گیری زمان
```

## لایسنس

همان لایسنس پروژه اصلی
