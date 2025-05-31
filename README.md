# 🎯 API Shooter

A fun real-time action game to learn REST API concepts through shooting gameplay! Built with Rust and designed for developers who want to master HTTP methods in an engaging way.

## 🎮 Game Overview

API Shooter transforms boring API documentation into an exciting space shooter experience. Players control a spaceship, scan enemies to discover their weaknesses, and attack using different HTTP methods (GET, POST, PUT, DELETE) while dodging incoming error code attacks!

## ✨ Features

- **Real-time action gameplay** with WASD movement and shooting
- **Educational REST API learning** through HTTP method combat
- **Strategic enemy weaknesses** requiring proper API method usage
- **Modern UI** with health bars and weapon management

## 🚀 Quick Start

### Prerequisites
- [Rust](https://rustup.rs/) (1.70 or later)

### Installation & Running

```bash
# Clone the repository
git clone https://github.com/your-username/api-shooter-cli.git
cd api-shooter-cli

# Run the game
cargo run

# Or install globally
cargo install --path .
api-shooter
```

## 🎯 How to Play

### Controls
- **WASD** or **Arrow Keys**: Move your spaceship
- **1**: Fire GET bullet (scan enemy for weakness)
- **2**: Fire POST bullet (medium damage)
- **3**: Fire PUT bullet (high damage)
- **4**: Fire DELETE bullet (ultimate attack, limited ammo)
- **ESC**: Quit game
- **SPACE**: Continue to next enemy (after victory)

### Gameplay Tips
1. **Always scan first**: Use GET (key 1) to reveal enemy weaknesses
2. **Exploit weaknesses**: Use the revealed weakness for critical hits
3. **Manage ammo**: DELETE is powerful but limited - use wisely
4. **Stay mobile**: Dodge incoming HTTP error attacks

## 🛠️ Built With

- **[Rust](https://www.rust-lang.org/)** - Systems programming language
- **[macroquad](https://macroquad.rs/)** - Simple game library
- **[rand](https://crates.io/crates/rand)** - Random number generation

---

**Made with  Rust for the Amazon Q CLI event**
- https://aws.amazon.com/jp/blogs/news/build-games-with-amazon-q-cli-and-score-a-t-shirt/
