# rustymonkytypey

[![rustymonkytypey](https://img.shields.io/badge/GitHub-rustymonkytypey-green?style=plastic)](https://github.com/wamaanachi/rustymonkytypey)

> monkeytype but inside your terminal so you don't have to open a bloated browser

---

## Features

- Word test modes (15, 25, 40, 50 words)
- Live WPM, raw WPM, and accuracy tracking
- Actual Braille speed graph on the results screen
- Mistake review table to see what you spelt wrong
- Theme switcher with live preview (Default, Gruvbox, Custom colors)
- Instant restart with Tab

## Stack

- **Frontend:** Ratatui / Terminal
- **Backend:** Rust
- **Engine:** Custom state machine
- **Styling:** Vibez

---

## Keybinds

Before typing:

- `Left / Right`: Choose word count (15, 25, 40, 50)
- `F2`: Open theme customization

While typing:

- `Space`: Next word / skip word
- `Backspace`: Undo character (or go back to previous word)
- `Tab`: Restart test with new words

On results screen:

- `R`: Review mistakes
- `Tab` / `Enter`: Start fresh test
- `Esc`: Quit

---

## Getting Started

### Prerequisites

Rust and cargo installed (`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`)

## Building from Source

(linux / mac / windows)

1. **Clone the repository:**

   ```shell
   git clone https://github.com/wamaanachi/rustymonkytypey.git
   cd rustymonkytypey
   ```

2. **Build and Run:**
   Probably just run:

   ```shell
   cargo run --release
   ```

3. **Or build the binary:**

   ```shell
   cargo build --release
   ./target/release/rustymonkytypey
   ```

4. **Add to alias (optional):**  
   If you want to run it from anywhere without typing `cargo run`:

If using Bash:

```shell
cp target/release/rustymonkytypey ~/.local/bin/
echo 'alias tuipe="~/.local/bin/rustymonkytypey"' >> ~/.bashrc
```

If using Zsh:

```shell
cp target/release/rustymonkytypey ~/.local/bin/
echo 'alias tuipe="~/.local/bin/rustymonkytypey"' >> ~/.zshrc
```

If using Fish:

```shell
cp target/release/rustymonkytypey ~/.local/bin/
abbr -a tuipe "~/.local/bin/rustymonkytypey"
```

## Contributors

- **[![Amaan](https://img.shields.io/badge/GitHub-MiniGun1239-orange?style=plastic)](https://www.github.com/MiniGun1239)**
- **[![Ruster](https://img.shields.io/badge/GitHub-Ruster-orange?style=plastic)](https://www.github.com/Tylerr-D)**
- **[![Chish](https://img.shields.io/badge/GitHub-Chish-orange?style=plastic)](https://github.com/chishxd)**

### Credits

- **[Monkeytype](https://monkeytype.com)** for the original idea and themes
- **[Ratatui](https://ratatui.rs)** for the terminal UI library
- **[Hack Club](https://hackclub.com)**

> Coded and tested on Linux, should work on any terminal with truecolor support.
