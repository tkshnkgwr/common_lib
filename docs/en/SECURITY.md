# Security Policy (SECURITY.md) - common_lib

**English** | [日本語版](../ja/SECURITY.md)

Security policy, supported versions, and component-specific security guidelines for `common_lib`.

---

## 1. Supported Versions

Security updates are provided only for the latest active release branch.

| Version | Supported |
| :--- | :--- |
| >= 0.1.x | Yes |
| < 0.1.0 | No |

---

## 2. Reporting a Vulnerability

If you discover a security vulnerability in `common_lib`, please **do not** open a public issue. Report it privately to the project owner/maintainer.

- **Response Time**: We will acknowledge your report within 48 hours and provide a detailed response along with potential next steps or mitigations.

---

## 3. Security Considerations

### Named Mutex Usage (Windows Single Instance Check)

The single-instance checks (`check_single_instance` and `desktop::acquire_single_instance`) utilize Windows Named Mutexes. Developers using these APIs should be aware of the following:

1. **Mutex Hijacking (Denial of Service)**:
   - Named Mutexes exist in a shared namespace. A malicious local process could pre-create a Mutex with the same name, preventing your application from launching (DoS).
   - **Recommendation**: Avoid generic or predictable Mutex names. We recommend incorporating a unique namespace prefix, such as a company domain name in reverse notation (e.g., `"com.mycompany.myapp.mutex_name"`), or a GUID.
2. **Access Control (Security Descriptor)**:
   - `CreateMutexW` in this library is initialized with default security descriptors (`None`). In multi-user environments (e.g., Terminal Services/Remote Desktop), this Mutex name is local to the user session by default.
   - If you want a system-wide single instance across all users, you would need to prefix the Mutex name with `"Global\\"` (e.g., `"Global\\my_unique_mutex"`), but this requires appropriate permissions. By default, names without a prefix are session-local (`"Local\\"` namespace).
