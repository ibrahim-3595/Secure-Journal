
# 🛡️ Secure Journal App

> *Your thoughts deserve the same protection as your passwords.*
> A beautifully minimal, end-to-end secure journal built with Rust — where privacy, performance, and design meet.

---

## ✨ Overview

**Secure Journal App** is a private journaling tool built in Rust that keeps your notes safe and encrypted.
Every entry is protected with **Argon2 password hashing** and **rpassword-based encryption**, ensuring that your data stays truly yours — even offline.

This isn’t just a journal — it’s a **fortress for your thoughts**.

---

## 🔐 Features

* 🧠 **Encrypted Entries** — Uses [`rpassword`](https://crates.io/crates/rpassword) to handle passwords securely without exposing them in terminal input.
* 🧩 **Strong Password Hashing** — Implements [`argon2`](https://crates.io/crates/argon2) to hash and verify master passwords.
* ⚡ **Async & Fast** — Powered by [`tokio`](https://crates.io/crates/tokio) for asynchronous, non-blocking operations.
* 🎨 **Colorized CLI Experience** — Beautiful terminal output with [`colorized`](https://crates.io/crates/colorized) for clear and intuitive interaction.
* 💾 **Next-Gen Database** — Uses [`SurrealDB`](https://surrealdb.com) for flexible and secure data storage.
* 🧰 **Robust Error Handling** — Managed with [`anyhow`](https://crates.io/crates/anyhow) for clear, user-friendly error messages.
* 📦 **Serialization Made Simple** — Data structures powered by [`serde`](https://crates.io/crates/serde) for seamless serialization and deserialization.

---

## ⚙️ Tech Stack

| Layer             | Technology             | Purpose                                    |
| :---------------- | :--------------------- | :----------------------------------------- |
| 🗄️ Database      | **SurrealDB**          | Secure, flexible data persistence          |
| 🔒 Encryption     | **rpassword + argon2** | Protects journal access and data integrity |
| ⚙️ Runtime        | **Tokio**              | Async operations & performance             |
| 🧰 Error Handling | **anyhow**             | Simplified and consistent error reporting  |
| 🧱 Serialization  | **Serde**              | Efficient and safe data handling           |
| 🎨 UI             | **Colorized**          | Clean and vivid command-line experience    |

---

## 🚀 Getting Started

### 1️⃣ Prerequisites

Make sure you have **Rust** (latest stable) installed.

```bash
rustup update
```

### 2️⃣ Clone the Repository

```bash
git clone https://github.com/yourusername/secure-journal.git
cd secure-journal
```

### 3️⃣ Build & Run

```bash
cargo build --release
cargo run
```

### 4️⃣ Create Your Master Password

The app will prompt you to set a secure password.
Your password is hashed with **Argon2**, and entries are encrypted using **rpassword** mechanisms.

---

## 🧱 Project Structure

```
secure-journal/
│
├── src/
│   ├── main.rs
│   ├── auth.rs
│   ├── db.rs
│   ├── menu.rs
│   └── models.rs
│
├── Cargo.toml
└── README.md
```

---

## 🔮 Future Plans

* [ ] Encrypted cloud sync option
* [ ] Databse and Storage enhancements
* [ ] Exporting journal as .pdf or .md
* [ ] Integrating Axum
* [ ] Adding a UI framework like Yew/Dioxus

---

## ❤️ Built With Rust

> *Fast. Safe. Fearless.*

---

## 🛠️ License

This project is licensed under the **MIT License** — see the [LICENSE](LICENSE) file for details.
