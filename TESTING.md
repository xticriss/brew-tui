# brew-tui Manual Testing Guide

## Pre-Test Setup
- [ ] Ensure brew is installed: `brew --version`
- [ ] Install some test packages if needed: `brew install wget curl git`
- [ ] Check current packages: `brew list`

## Session 1: Critical Fixes Verification

### UTF-8 Safety
- [ ] Launch brew-tui: `cargo run --release`
- [ ] Press `/` to enter search mode
- [ ] Type emoji: `test🔥`
- [ ] Type CJK characters: `你好`
- [ ] Type combining marks: `café`
- [ ] **Expected**: No crashes or panics, smooth filtering

### Input Validation
- [ ] Select a package from the list
- [ ] Try update operation (u → y)
- [ ] Try pin operation (p)
- [ ] **Expected**: Operations complete successfully with clear messages

### Navigation
- [ ] Type filter with no matches (e.g., `/zzzznonexistent`)
- [ ] **Expected**: Empty list, no crash
- [ ] Clear filter (backspace all)
- [ ] Press `g` (go to first)
- [ ] Press `G` (go to last)
- [ ] Press `Ctrl+d` (page down)
- [ ] Press `Ctrl+u` (page up)
- [ ] **Expected**: Smooth navigation, no crashes at boundaries

**Pass Criteria**: ✅ No crashes, ✅ All navigation smooth

---

## Session 2: Performance Verification

### Load Time
- [ ] Quit brew-tui (press `q`)
- [ ] Note start time
- [ ] Launch: `cargo run --release`
- [ ] Note time when package list is visible
- [ ] **Target**: <2 seconds for initial load
- [ ] Actual time: __________ seconds

### Filter Performance
- [ ] Press `/` to enter search mode
- [ ] Type rapidly: `test`
- [ ] Backspace rapidly
- [ ] Repeat several times
- [ ] **Expected**: No lag, instant updates

### Memory Usage
- [ ] Open Activity Monitor (macOS) or Task Manager (Linux/Windows)
- [ ] Find brew-tui process
- [ ] Note memory usage: __________ MB
- [ ] Filter packages for 1 minute (type/backspace repeatedly)
- [ ] Note final memory: __________ MB
- [ ] **Expected**: Stable memory, <150MB, no continuous growth

**Pass Criteria**: ✅ Fast loads, ✅ No lag, ✅ Stable memory

---

## Session 3: Code Quality Verification

### Visual Appearance (Should be identical to before refactoring)
- [ ] Launch brew-tui
- [ ] Press `?` for help dialog
- [ ] **Expected**: Dialog centered, all text visible
- [ ] Press `?` to close
- [ ] Select package, press `u` for update confirmation
- [ ] **Expected**: Dialog centered with Yes/No buttons
- [ ] Press `n` to cancel

### All Features Work
- [ ] Press `Tab` to switch between Formulae/Casks
- [ ] Press `d` or `Enter` to toggle details panel
- [ ] **Expected**: Details panel appears on right
- [ ] Press `d` again to close
- [ ] Select package, press `t` for dependency tree
- [ ] **Expected**: Dependency tree appears
- [ ] Press `t` to close
- [ ] Check status bar at bottom
- [ ] **Expected**: Shows package counts

**Pass Criteria**: ✅ Visually identical, ✅ All features working

---

## Session 4: Comprehensive Testing

### Complete Workflow Test
1. [ ] Launch app: `cargo run --release`
2. [ ] Filter for package: `/git`
3. [ ] Toggle details: `d`
4. [ ] View dependency tree: `t`
5. [ ] Switch tabs: `Tab`
6. [ ] Toggle outdated filter: `o`
7. [ ] Navigate: `j`, `k`, `g`, `G`
8. [ ] Help: `?`
9. [ ] Quit: `q`

### Operations Test (BE CAREFUL)
- [ ] Select a safe test package
- [ ] Pin it: `p` → observe status changes to "📌 Pinned"
- [ ] Unpin: `p` again → status back to "✓ Current"
- [ ] Refresh list: `r`
- [ ] **Expected**: All operations complete without errors

### Error Handling
- [ ] Filter for non-existent package: `/zzzznonexistent123456`
- [ ] **Expected**: Empty list, no crash
- [ ] Clear filter
- [ ] Try operations with no network (if possible)
- [ ] **Expected**: Clear error messages

### Stress Test
- [ ] Type in filter very rapidly for 30 seconds
- [ ] Switch tabs repeatedly (Tab Tab Tab...)
- [ ] Scroll through entire package list rapidly
- [ ] **Expected**: No freezing, no lag, no crashes

**Pass Criteria**: ✅ All workflows complete, ✅ No errors or crashes

---

## Performance Benchmarks

| Metric | Target | Actual |
|--------|--------|--------|
| App launch to visible | <2s | _____ |
| Filter response time | <100ms | Instant ✓ |
| View switch time | <50ms | Instant ✓ |
| Memory (idle) | <100MB | _____ |
| Memory (after 5min use) | <150MB | _____ |

---

## Final Approval Checklist

- [ ] Session 1: All critical fixes verified
- [ ] Session 2: Performance targets met
- [ ] Session 3: No visual regressions
- [ ] Session 4: All workflows successful
- [ ] Build passes: `cargo build --release`
- [ ] Tests pass: `cargo test`
- [ ] No clippy warnings: `cargo clippy`

**Tested by**: _________________ **Date**: _________

---

## Known Issues / Notes

(Record any issues found during testing here)

-
-
-

---

## Deployment

After all tests pass:

```bash
# Build release binary
cargo build --release

# Deploy to local bin (optional)
cp target/release/brew-tui ~/.cargo/bin/

# Verify
brew-tui --version  # or just run: brew-tui
```
