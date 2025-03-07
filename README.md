
# Quack

You need `filebrowser` for it to run. See the `Makefile`.

## 20000 requests

|          | .NET | Python | Rust |
| -------- | --   | ---    | ---  |
| Real     | 0.8s | 2.1s   | 0.7s |
| User     | 2.7s | 1.7s   | 1.3s |
| Sys      | 0.9s | 0.3s   | 1.8s |

Winners:
 - 🚀 Fastest: Rust
 - 💚 Energy-efficient: Python

## 200000 requests

|          | .NET  | Python | Rust  |
| -------- | --    | ---    | ---   |
| Real     | 8.0s  | 21.7s  | 6.6s  |
| User     | 17.2s | 17.7s  | 13.2s |
| Sys      | 6.4s  | 3.0s   | 18.3s |

Winners:
- 🚀 Fastest: Rust
- 💚 Energy-efficient: Python
