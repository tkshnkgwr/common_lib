# Performance & Footprints (FOOTPRINTS.md) - common_lib

**English** | [日本語版](../ja/FOOTPRINTS.md)

Performance metrics, memory footprint, binary size impact, and algorithmic complexity notes for `common_lib`.

---

## 1. Library Footprint

### Memory Footprint
- **Single Instance Guard**:
  - `check_single_instance` and `acquire_single_instance` consume a single Windows Kernel Mutex resource.
  - Process memory footprint impact is minimal (< a few KB).
- **Text Diff Engine**:
  - During `compute_diff` execution for $N$ lines (old text) and $M$ lines (new text), a DP table of size $(N+1) \times (M+1)$ `usize` entries is allocated on the heap.
  - Example: A 100 line vs 100 line diff temporarily consumes ~81 KB of memory.

### Binary Size Impact
- Compiled as an `rlib`.
- Dependency on `serde` adds minimal size overhead (a few dozen KB) as only needed features are linked.
- Dependency on `windows` crate links only essential Win32 APIs (`CreateMutexW`, `CloseHandle`), incurring zero unnecessary size overhead.

---

## 2. Algorithmic Complexity

| Feature / API | Time Complexity | Space Complexity | Notes |
| :--- | :--- | :--- | :--- |
| `check_single_instance` | $\mathcal{O}(1)$ | $\mathcal{O}(1)$ | Kernel object creation overhead. |
| `acquire_single_instance` | $\mathcal{O}(1)$ | $\mathcal{O}(1)$ | Guard acquisition. |
| `add` | $\mathcal{O}(1)$ | $\mathcal{O}(1)$ | Register addition. |
| `count_occurrences` | $\mathcal{O}(L)$ | $\mathcal{O}(L)$ | $L$ = byte length of text. |
| `format_bytes` | $\mathcal{O}(1)$ | $\mathcal{O}(1)$ | String formatting. |
| `suggest_tags` | $\mathcal{O}(K \times L)$ | $\mathcal{O}(L)$ | $K$ = candidate tags count, $L$ = text length. |
| `compute_diff` | $\mathcal{O}(N \times M)$ | $\mathcal{O}(N \times M)$ | LCS dynamic programming algorithm. |

---

## 3. Recommended Release Profile Settings

To optimize binary size and execution speed when consuming `common_lib` in application binaries:

```toml
[profile.release]
opt-level = 3       # Maximum optimization
lto = true          # Link Time Optimization
codegen-units = 1   # Maximize LTO efficiency
panic = "abort"     # Disable unwinding for size reduction
strip = true        # Strip debug symbols
```
