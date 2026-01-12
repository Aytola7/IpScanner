# مقایسه نسخه Python و Rust

## 📊 مقایسه ویژگی‌ها

| ویژگی | Python (اصلی) | Rust (جدید) |
|-------|---------------|-------------|
| **سرعت اجرا** | متوسط | 🚀 خیلی سریع (2-5x) |
| **مصرف حافظه** | بالا | 💾 خیلی کم |
| **همزمانی** | asyncio + threading | Tokio (pure async) |
| **Type Safety** | Dynamic typing | Static + Strong typing |
| **خطایابی** | Runtime errors | Compile-time checks ✅ |
| **Null Safety** | None checks needed | Option<T> و Result<T,E> |
| **Performance** | Interpreted | Compiled native code |

## 🏗️ معماری

### Python Version
```
runer.py (main)
└── worker/
    ├── engine.py         → WorkerSupervisor & watchdog
    ├── base_worker.py    → BaseWorker (ABC)
    ├── task_handler.py   → AppTaskHandler
    ├── createip.py       → IP parsing & sampling
    ├── timer.py          → Timer class
    └── logger.py         → Logging setup
```

### Rust Version
```
main.rs
└── src/
    ├── worker_engine.rs   → WorkerEngine + Supervisor
    ├── task_handler.rs    → AppTaskHandler
    ├── ip_creator.rs      → IP parsing & sampling
    └── timer.rs           → Timer struct
```

## 📝 تفاوت‌های کلیدی

### 1. Type System

**Python:**
```python
def handle(self, item):  # item می‌تونه هر چیزی باشه
    pass
```

**Rust:**
```rust
async fn handle(&self, ip: String) -> Result<()> {
    // فقط String قبول می‌کنه، خطا رو در Result برمی‌گردونه
}
```

### 2. Error Handling

**Python:**
```python
try:
    result = do_something()
except Exception as e:
    logger.error(f"Error: {e}")
```

**Rust:**
```rust
match do_something().await {
    Ok(result) => { /* success */ },
    Err(e) => error!("Error: {}", e),
}
// یا استفاده از ? operator
let result = do_something().await?;
```

### 3. Concurrency Model

**Python:**
```python
# ترکیب threading + asyncio
loop = asyncio.new_event_loop()
thread = threading.Thread(target=worker.run)
```

**Rust:**
```rust
// Pure async با Tokio
tokio::spawn(async move {
    worker.run().await;
});
```

### 4. Memory Management

**Python:**
- Garbage Collection (GC)
- Reference counting
- Overhead زیاد

**Rust:**
- Ownership system
- Zero-cost abstractions
- بدون GC، بدون overhead

### 5. Null Safety

**Python:**
```python
result = None  # ممکنه بعداً NoneType error بده
if result is not None:
    do_something(result)
```

**Rust:**
```rust
let result: Option<String> = None;
if let Some(value) = result {
    do_something(value);
}
// یا با unwrap_or, unwrap_or_else, etc.
```

## ⚡ Performance Benchmarks (تخمینی)

| عملیات | Python | Rust | بهبود |
|--------|--------|------|-------|
| استارت 100 worker | ~2s | ~0.1s | 20x |
| پردازش 10K IP | ~300s | ~60-120s | 2.5-5x |
| مصرف RAM | ~500MB | ~50MB | 10x |

## 🎯 مزایای Rust

1. ✅ **سرعت بالا**: کامپایل به machine code
2. ✅ **ایمنی حافظه**: بدون memory leaks
3. ✅ **Concurrency ایمن**: بدون data races
4. ✅ **Type safety**: خطاها در compile time
5. ✅ **مصرف منابع کم**: مناسب برای production
6. ✅ **Portable**: یک binary مستقل

## 🔧 مزایای Python

1. ✅ **توسعه سریع**: کد کمتر، development سریع‌تر
2. ✅ **Ecosystem غنی**: کتابخانه‌های بیشتر
3. ✅ **یادگیری راحت‌تر**: Syntax ساده‌تر
4. ✅ **REPL و debugging**: تست سریع

## 🚀 استفاده از نسخه Rust

### Build
```bash
cd ip_scanner_rust
cargo build --release
```

### Run
```bash
# استفاده ساده
cargo run --release

# با تنظیمات
cargo run --release -- --workers 50 --concurrency 20
```

### Install
```bash
cargo install --path .
# سپس می‌تونید مستقیماً اجرا کنید:
ip_scanner_rust --workers 100 --concurrency 30
```

## 📦 Dependencies

### Python
- asyncio (built-in)
- threading (built-in)
- ipaddress (built-in)
- ⚠️ نیاز به Python interpreter

### Rust
- tokio (async runtime)
- clap (CLI parsing)
- ipnetwork (IP utilities)
- log + env_logger
- ✅ بدون dependency خارجی در runtime

## 🎓 نتیجه‌گیری

**Python را انتخاب کنید اگر:**
- سرعت توسعه مهم‌تر از performance است
- نیاز به rapid prototyping دارید
- تیم با Python آشناتر است

**Rust را انتخاب کنید اگر:**
- Performance و سرعت اجرا مهم است
- مصرف منابع (RAM/CPU) محدود است
- نیاز به stability و reliability بالا دارید
- می‌خواهید binary مستقل بسازید
- در production با scale بالا استفاده می‌شود
