# 🦀 Rust AI Debugger CLI

A powerful **Command Line AI Debugging Tool written in Rust** that analyzes code files and identifies bugs, issues, and improvements using the **Gemini API (gemini-3-flash-preview model)**.

This tool helps developers quickly debug and improve their code.

This project is part of my **Rust learning journey (Day 14 Project)**.

---

## 🚀 Features

* Detect bugs in code
* Suggest fixes and improvements
* Analyze any code file
* Uses Gemini 3 Flash model
* Secure API key using `.env`
* Lightweight CLI tool

---

## 🛠 Built With

* **Rust**
* `reqwest`
* `serde_json`
* `dotenv`
* Google Gemini API

---

## 📂 Project Structure

```text
ai_debugger_14/
│
├── src/
│   └── main.rs
├── Cargo.toml
├── Cargo.lock
├── .gitignore
├── .env
├── .env.example
```

---

## ⚙️ Installation

```bash
git clone https://github.com/yourusername/rust-ai-debugger.git
cd rust-ai-debugger
cargo build
```

---

## 🔑 Setup

Create `.env` file:

```env
GEMINI_API_KEY=your_api_key_here
```

---

## ▶️ Usage

```bash
cargo run
```

Enter file path when prompted.

---

## 📸 Example

<img width="1723" height="776" alt="image" src="https://github.com/user-attachments/assets/227712c3-d4bb-4113-908b-35d51a239d46" />

---

## 🧠 Concepts Practiced

* File handling
* AI-based debugging
* Prompt engineering
* API integration
* CLI design

---

## 🔮 Future Improvements

* Multi-file debugging
* Language detection
* Auto-fix suggestions
* Export debug report
* Integration with Git

---

## 📜 License

MIT License

---

## 👨‍💻 Author

**Khurram Rashid**  
B.Tech Computer Science Engineering  
Amity University Mumbai
