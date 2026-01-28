# Troubleshooting Guide

**Common issues and their solutions**

---

## Quick Diagnosis

```bash
# Run this diagnostic script
cd ~/.config/hypr/hyprricer

# 1. Check if CLI is built
./target/release/hyprricer --version 2>/dev/null || echo "❌ CLI not built"

# 2. Check if daemon is running
pgrep -f hrm_daemon && echo "✅ Daemon running" || echo "❌ Daemon not running"

# 3. Check config file
[ -f live/active_session.conf ] && echo "✅ Config exists" || echo "❌ Config missing"

# 4. View recent logs
tail -20 live/daemon.log 2>/dev/null || echo "❌ No logs found"
```

---

## Build Errors

### Error: `Missing dependency for Watcher 'cpu_usage'`

**Symptom:**
```
❌ Missing dependency for Watcher (check #1) 'cpu_usage'.
   Command 'which top' failed.
```

**Cause:** Required command-line tool not installed.

**Solution:**
```bash
# Install the missing tool
sudo pacman -S procps-ng  # Arch
sudo apt install procps   # Ubuntu/Debian

# Verify
which top
```

**Why this happens:** hyprricer validates dependencies at build-time. Each watcher/provider can specify `check` commands.

---

### Error: `Dynamic logic file not found`

**Symptom:**
```
Dynamic logic file not found: "/path/to/themes/mytheme/logic/style.rs"
```

**Cause:** Theme references a logic file that doesn't exist.

**Solution:**
```bash
# Check if file exists
ls themes/mytheme/logic/style.rs

# If missing, create it
touch themes/mytheme/logic/style.rs
```

**Template for a minimal logic file:**
```rust
use crate::Context;

pub fn resolve(ctx: &Context) -> String {
    "# Placeholder".to_string()
}
```

---

### Error: `Failed to parse TOML`

**Symptom:**
```
Failed to parse TOML: /path/to/file.toml
TOML parse error at line 5, column 10
  |
5 | check = which jq
  |         ^^^^^^^^^
expected value, found identifier `which`
```

**Cause:** Syntax error in TOML file.

**Solution:**
Strings must be quoted:
```toml
# ❌ Wrong
check = which jq

# ✅ Correct
check = "which jq"

# ✅ Also correct (for multiple checks)
check = ["which jq", "which hyprctl"]
```

---

## Runtime Errors

### Daemon Not Starting

**Symptom:**
```bash
ps aux | grep hrm_daemon
# No output
```

**Diagnosis:**
```bash
# Try running daemon manually
cd ~/.config/hypr/hyprricer
./live/daemon

# Check for errors
```

**Common causes:**

1. **Daemon was never compiled**
   ```bash
   # Solution: Rebuild
   hyprricer build --profile seiki
   cd generated/source
   cargo build --release
   cp target/release/hrm_daemon ../../live/daemon
   ```

2. **Missing permissions**
   ```bash
   # Solution: Fix permissions
   chmod +x live/daemon
   ```

3. **Port/socket conflict**
   ```bash
   # Solution: Kill old daemon
   pkill -f hrm_daemon
   ./live/daemon
   ```

---

### Config Not Updating

**Symptom:** You make changes but Hyprland doesn't reflect them.

**Diagnosis:**
```bash
# Check if config file is being updated
watch -n 1 stat live/active_session.conf
```

**Solutions:**

1. **Hyprland not sourcing the file**
   ```bash
   # Check your hyprland.conf
   grep "hyprricer" ~/.config/hypr/hyprland.conf
   
   # Should show:
   # source = ~/.config/hypr/hyprricer/live/active_session.conf
   ```

2. **Daemon not receiving events**
   ```bash
   # Check daemon logs
   tail -f live/daemon.log
   
   # You should see:
   # ✨ Event: Event { key: "cpu_usage", value: "45" }
   # 💾 Config Updated
   ```

3. **Manual reload needed**
   ```bash
   hyprctl reload
   ```

---

### Watchers Not Triggering

**Symptom:** Logs show daemon running but no events:
```
🍚 Daemon Started for 'My Theme'
   👂 Waiting for events...
# ... nothing else
```

**Diagnosis:**
```bash
# Check if watchers are even defined
cat generated/source/src/main.rs | grep "async fn watch_"
```

**Solutions:**

1. **Theme doesn't use any watchers**
   
   Check `theme.toml`:
   ```toml
   inputs = ["cpu_usage", "time_part"]  # Must list watchers
   ```

2. **Watcher commands failing silently**
   
   Test manually:
   ```bash
   # From registry definition
   top -bn1 | grep 'Cpu(s)' | sed 's/.*, *\([0-9.]*\)%* id.*/\1/' | awk '{print 100 - $1}'
   ```

3. **Interval too long**
   
   Edit `catalog/registry/hardware.toml`:
   ```toml
   [watcher.cpu_usage]
   interval = 1000  # Reduce from 5000 to 1000ms
   ```

---

## Compilation Errors

### Error: `cannot find type Context in this scope`

**Symptom:**
```rust
error[E0412]: cannot find type `Context` in this scope
 --> themes/mytheme/logic/style.rs:3:23
  |
3 | pub fn resolve(ctx: &Context) -> String {
  |                       ^^^^^^^ not found in this scope
```

**Cause:** Missing import in logic file.

**Solution:**
Add to top of your logic file:
```rust
use crate::Context;

pub fn resolve(ctx: &Context) -> String {
    // ...
}
```

---

### Error: `mismatched types`

**Symptom:**
```rust
error[E0308]: mismatched types
  --> logic/style.rs:8:5
   |
8  |     42
   |     ^^ expected struct `String`, found integer
```

**Cause:** Logic function must return `String`, not other types.

**Solution:**
Convert to string:
```rust
// ❌ Wrong
pub fn resolve(ctx: &Context) -> String {
    42
}

// ✅ Correct
pub fn resolve(ctx: &Context) -> String {
    "42".to_string()
    // or
    format!("{}", 42)
}
```

---

## Performance Issues

### High CPU Usage

**Diagnosis:**
```bash
# Monitor CPU usage
top -p $(pgrep hrm_daemon)
```

**Causes & Solutions:**

1. **Too many watchers with short intervals**
   ```toml
   # ❌ Bad: Polling every 100ms
   interval = 100
   
   # ✅ Good: Polling every 1-5 seconds
   interval = 2000
   ```

2. **Heavy commands in watchers**
   ```bash
   # ❌ Bad: Spawns many processes
   cmd = "find / -name '*.log' | wc -l"
   
   # ✅ Good: Simple, fast command
   cmd = "date +%H"
   ```

3. **Provider timeouts not working**
   
   Providers should complete in <200ms. If they don't, they're killed, but check if your commands are too slow:
   ```bash
   time hyprctl activewindow -j | jq -r .class
   # Should be <50ms
   ```

---

### Memory Leaks

**Diagnosis:**
```bash
# Monitor memory over time
watch -n 5 'ps aux | grep hrm_daemon | grep -v grep'
```

**Solution:** Memory leaks are unlikely in Rust, but if you see growth:

1. Restart daemon periodically (shouldn't be necessary, but):
   ```bash
   pkill hrm_daemon
   ~/.config/hypr/hyprricer/live/daemon &
   ```

2. Report as a bug with:
   ```bash
   # Memory info
   cat /proc/$(pgrep hrm_daemon)/status | grep VmRSS
   
   # Logs
   tail -100 live/daemon.log
   ```

---

## Theme-Specific Issues

### Seiki Theme: Borders Not Changing Color

**Check:**
1. CPU watcher working?
   ```bash
   tail -f live/daemon.log | grep cpu_usage
   ```

2. Logic function being called?
   ```bash
   cat live/active_session.conf | grep active_border
   ```

3. Hyprland reloading?
   ```bash
   hyprctl reload
   ```

---

### Modern Dark: Static Components Not Loading

**Check:**
1. Registry paths correct?
   ```bash
   cat catalog/registry/apps.toml
   # Verify 'path' points to existing file
   ```

2. File actually exists?
   ```bash
   cat catalog/static/apps/apps_modern.conf
   ```

3. Referenced correctly in theme?
   ```toml
   [static]
   apps = "apps_modern"  # Must match registry key
   ```

---

## Advanced Debugging

### Inspecting Generated Code

The generated daemon source is available for inspection:

```bash
# Main daemon code
cat generated/source/src/main.rs

# Your injected logic
cat generated/source/src/logic/style.rs

# Compilation errors
cd generated/source
cargo build --release 2>&1 | less
```

### Enabling Debug Logging

**In daemon:**
Currently logs are always enabled. For more verbose output, modify generated code:
```rust
// In generated/source/src/main.rs
println!("DEBUG: cache = {:?}", cache);
```

**In CLI:**
```bash
RUST_LOG=debug hyprricer build --profile seiki
```

### Manual Daemon Control

```bash
# Stop daemon
pkill hrm_daemon

# Start daemon in foreground (see output directly)
cd ~/.config/hypr/hyprricer
./live/daemon

# Start in background
./live/daemon &

# Check if running
pgrep -a hrm_daemon
```

---

## Getting More Help

### Before Opening an Issue

1. **Gather information:**
   ```bash
   # System info
   uname -a
   hyprctl version
   rustc --version
   
   # hyprricer state
   ls -la ~/.config/hypr/hyprricer/live/
   tail -50 ~/.config/hypr/hyprricer/live/daemon.log
   
   # Last build output
   hyprricer build --profile yourprofile 2>&1 | tee build.log
   ```

2. **Minimal reproduction:**
   - Can you reproduce with `modern_dark` theme?
   - Does it happen with a fresh `hyprricer` clone?

3. **Search existing issues:**
   - [GitHub Issues](https://github.com/yourusername/hyprricer/issues)

### Reporting a Bug

Include:
- OS and version
- Hyprland version
- hyprricer version (git commit hash)
- Full error message
- Relevant logs
- Steps to reproduce

Use this template:
```markdown
**System:**
- OS: Arch Linux
- Hyprland: 0.35.0
- hyprricer: commit abc1234

**Issue:**
Daemon crashes when CPU usage > 90%

**Logs:**
```
[paste daemon.log here]
```

**Steps to Reproduce:**
1. Build seiki theme
2. Run `stress -c 8`
3. Daemon crashes after 30s
```

---

**Still stuck?** Ask in [Discussions](https://github.com/yourusername/hyprricer/discussions)!
