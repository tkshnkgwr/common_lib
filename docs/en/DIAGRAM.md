# System Diagrams (DIAGRAM.md) - common_lib

**English** | [日本語版](../ja/DIAGRAM.md)

Diagrams showing the module structure, single-instance execution sequence, and diff calculation data flow of `common_lib`.

---

## 1. Module and API Structure

```mermaid
graph TD
    subgraph common_lib
        direction TB
        lib["lib.rs (Root)"]
        desktop["desktop.rs"]
        text["text.rs"]
        error["error.rs"]
    end

    lib -->|"Expose / Re-export"| desktop
    lib -->|"Expose / Re-export"| text
    lib -->|"Expose / Re-export"| error
    lib -->|"Numeric Addition"| add["add"]

    desktop -->|"Single Instance Check"| check_single_instance["check_single_instance"]
    desktop -->|"Guard Check"| acquire_single_instance["acquire_single_instance"]
    desktop -->|"Resource Guard"| SingleInstanceGuard["SingleInstanceGuard (RAII Guard)"]

    text -->|"Count Occurrences"| count_occurrences["count_occurrences"]
    text -->|"Text Diff Calculation"| compute_diff["compute_diff"]
    text -->|"Format Byte Size"| format_bytes["format_bytes"]
    text -->|"Suggest Tags"| suggest_tags["suggest_tags"]

    suggest_tags -.->|"Depends on"| count_occurrences

    error -->|"Custom Error"| Error["Error enum"]
    error -->|"Result Type"| Result["Result type alias"]
```

---

## 2. Single Instance Execution Sequence (Desktop Guard Mode)

The lifecycle of single instance execution guard using `acquire_single_instance` on Windows:

```mermaid
sequenceDiagram
    autonumber
    participant App as Application
    participant Lib as common_lib (desktop)
    participant OS as OS (Windows Kernel)

    App->>Lib: acquire_single_instance(mutex_name)
    Lib->>OS: CreateMutexW(mutex_name)
    
    alt Fresh Launch (Mutex creation succeeded)
        OS-->>Lib: Valid Handle
        Lib-->>App: Some(SingleInstanceGuard)
        Note over App: Normal application execution
        
        App->>Lib: Guard Object Dropped (Out of Scope)
        Lib->>OS: CloseHandle(handle)
        Note over OS: Mutex Resource Released
    else Duplicate Launch (Mutex already exists)
        OS-->>Lib: ERROR_ALREADY_EXISTS
        Lib->>OS: CloseHandle(handle)
        Lib-->>App: None
        Note over App: Abort launch and exit
    end
```

---

## 3. Text Diff (LCS) Data Flow

The processing flow for `compute_diff` to extract line-by-line diffs:

```mermaid
graph TD
    old["old_text"] --> split_old["Split Lines (old_lines)"]
    new["new_text"] --> split_new["Split Lines (new_lines)"]
    
    split_old --> dp["Build LCS DP Table<br/>Calculate dp[i][j]"]
    split_new --> dp
    
    dp --> backtrack["Backtrack Path<br/>(Traverse i, j pointers)"]
    
    backtrack -->|"Match"| unchanged["DiffType::Unchanged"]
    backtrack -->|"New text only"| added["DiffType::Added"]
    track_del["Old text only"] -->|"Old text only"| removed["DiffType::Removed"]
    backtrack -.-> track_del
    
    unchanged --> merge["Collect to Result Vector (Vec&lt;DiffPart&gt;)"]
    added --> merge
    removed --> merge
    
    merge --> out["Final Output"]
```
