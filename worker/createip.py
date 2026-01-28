from pathlib import Path
import ipaddress
import random
import math

def createip():
    PROJECT_ROOT = Path(".").resolve()
    IP_FILE = PROJECT_ROOT / "ip4.txt" 

    ip_lines = [l.strip() for l in IP_FILE.read_text(encoding="utf-8").splitlines() if l.strip()]

    def expand_ip_line(line: str):
        line = line.strip()
        if not line:
            return []
        if "/" in line:
            try:
                net = ipaddress.ip_network(line, strict=False)
                return [str(ip) for ip in net.hosts()] if net.num_addresses > 1 else [str(next(net.hosts()))]
            except Exception:
                return []
        if "-" in line:
            try:
                start_s, end_s = line.split("-", 1)
                start_ip = ipaddress.ip_address(start_s.strip())
                end_ip = ipaddress.ip_address(end_s.strip())
                if int(end_ip) < int(start_ip):
                    return []
                ips = []
                cur = int(start_ip)
                endi = int(end_ip)
                MAX_RANGE = 65536
                if (endi - cur) > MAX_RANGE:
                    raise ValueError("Range too large")
                while cur <= endi:
                    ips.append(str(ipaddress.ip_address(cur)))
                    cur += 1
                return ips
            except Exception:
                return []
        try:
            ipaddress.ip_address(line)
            return [line]
        except Exception:
            return []

    def is_single_ip_line(line: str) -> bool:
        s = line.strip()
        if not s or "/" in s or "-" in s:
            return False
        try:
            ipaddress.ip_address(s)
            return True
        except Exception:
            return False

    def ask_int(prompt: str, default_val: int, min_val: int = 0, max_val: int = 10_000_000) -> int:
        try:
            raw = input(f"{prompt} [{default_val}]: ").strip()
            val = int(raw) if raw else default_val
            return max(min_val, min(val, max_val))
        except Exception:
            return default_val

    groups_raw = []
    for ln in ip_lines:
        ex = expand_ip_line(ln)
        if not ex:
            print(f"Warning: couldn't parse IP line: {ln}")
            continue
        is_single = is_single_ip_line(ln)
        if not is_single:
            any_range_line = True
            all_single_ip_lines = False
        groups_raw.append({"label": ln, "ips": ex, "single_line": is_single})

    # بررسی اینکه آیا Range یا CIDR داریم یا نه
    has_ranges = any(not g["single_line"] for g in groups_raw)
    
    if has_ranges:
        percent = ask_int("What percent of IPs to test? (0-100)", 100, 0, 100)
    else:
        percent = 100
        print("All single IPs will be tested (no sampling for individual IPs)")

    final_groups = []

    for g in groups_raw:
        ips = g["ips"]
        n = len(ips)
        if n == 0:
            continue
        
        # برای single IP ها، همه رو حفظ می‌کنیم
        if g["single_line"]:
            sampled = ips
            label_suffix = ""
        else:
            # برای CIDR و Range ها، نمونه‌گیری می‌کنیم
            if percent >= 100:
                sampled = ips
            else:
                k = max(1, math.ceil(n * (percent / 100.0)))
                k = min(k, n)
                sampled = ips if k == n else random.sample(ips, k)
            label_suffix = f"  (sample {percent}%)"
        
        final_groups.append({"label": g["label"] + label_suffix, "ips": sampled})

    return final_groups