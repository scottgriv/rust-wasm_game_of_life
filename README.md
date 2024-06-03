<!-- Begin README -->

<div align="center">
    <a href="https://scottgriv.github.io/rust-wasm_game_of_life/" target="_blank">
        <img src="./docs/images/cgol.gif" width="200" height="200"/>
    </a>
</div>
<br>
<p align="center">
    <a href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/Rust-1.74.0-000000?style=for-the-badge&logo=rust" alt="Rust Badge" /></a>
    <br>
    <a href="https://github.com/scottgriv"><img src="https://img.shields.io/badge/github-follow_me-181717?style=for-the-badge&logo=github&color=181717" alt="GitHub Badge" /></a>
    <a href="mailto:scott.grivner@gmail.com"><img src="https://img.shields.io/badge/gmail-contact_me-EA4335?style=for-the-badge&logo=gmail" alt="Email Badge" /></a>
    <a href="https://www.buymeacoffee.com/scottgriv"><img src="https://img.shields.io/badge/buy_me_a_coffee-support_me-FFDD00?style=for-the-badge&logo=buymeacoffee&color=FFDD00" alt="BuyMeACoffee Badge" /></a>
    <br>
     <a href="https://github.com/scottgriv/Palm-Tree/actions/workflows/pages/pages-build-deployment" target="_blank"><img alt="GitHub Workflow Status (with event)" src="https://img.shields.io/github/actions/workflow/status/scottgriv/Palm-Tree/pages%2Fpages-build-deployment?style=for-the-badge&logo=github&label=GitHub%20Pages"></a>
    <a href="https://prgportfolio.com" target="_blank"><img src="https://img.shields.io/badge/PRG-Bronze Project-CD7F32?style=for-the-badge&logo=data:image/svg%2bxml;base64,PD94bWwgdmVyc2lvbj0iMS4wIiBzdGFuZGFsb25lPSJubyI/Pgo8IURPQ1RZUEUgc3ZnIFBVQkxJQyAiLS8vVzNDLy9EVEQgU1ZHIDIwMDEwOTA0Ly9FTiIKICJodHRwOi8vd3d3LnczLm9yZy9UUi8yMDAxL1JFQy1TVkctMjAwMTA5MDQvRFREL3N2ZzEwLmR0ZCI+CjxzdmcgdmVyc2lvbj0iMS4wIiB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciCiB3aWR0aD0iMjYuMDAwMDAwcHQiIGhlaWdodD0iMzQuMDAwMDAwcHQiIHZpZXdCb3g9IjAgMCAyNi4wMDAwMDAgMzQuMDAwMDAwIgogcHJlc2VydmVBc3BlY3RSYXRpbz0ieE1pZFlNaWQgbWVldCI+Cgo8ZyB0cmFuc2Zvcm09InRyYW5zbGF0ZSgwLjAwMDAwMCwzNC4wMDAwMDApIHNjYWxlKDAuMTAwMDAwLC0wLjEwMDAwMCkiCmZpbGw9IiNDRDdGMzIiIHN0cm9rZT0ibm9uZSI+CjxwYXRoIGQ9Ik0xMiAzMjggYy04IC04IC0xMiAtNTEgLTEyIC0xMzUgMCAtMTA5IDIgLTEyNSAxOSAtMTQwIDQyIC0zOCA0OAotNDIgNTkgLTMxIDcgNyAxNyA2IDMxIC0xIDEzIC03IDIxIC04IDIxIC0yIDAgNiAyOCAxMSA2MyAxMyBsNjIgMyAwIDE1MCAwCjE1MCAtMTE1IDMgYy04MSAyIC0xMTkgLTEgLTEyOCAtMTB6IG0xMDIgLTc0IGMtNiAtMzMgLTUgLTM2IDE3IC0zMiAxOCAyIDIzCjggMjEgMjUgLTMgMjQgMTUgNDAgMzAgMjUgMTQgLTE0IC0xNyAtNTkgLTQ4IC02NiAtMjAgLTUgLTIzIC0xMSAtMTggLTMyIDYKLTIxIDMgLTI1IC0xMSAtMjIgLTE2IDIgLTE4IDEzIC0xOCA2NiAxIDc3IDAgNzIgMTggNzIgMTMgMCAxNSAtNyA5IC0zNnoKbTExNiAtMTY5IGMwIC0yMyAtMyAtMjUgLTQ5IC0yNSAtNDAgMCAtNTAgMyAtNTQgMjAgLTMgMTQgLTE0IDIwIC0zMiAyMCAtMTgKMCAtMjkgLTYgLTMyIC0yMCAtNyAtMjUgLTIzIC0yNiAtMjMgLTIgMCAyOSA4IDMyIDEwMiAzMiA4NyAwIDg4IDAgODggLTI1eiIvPgo8L2c+Cjwvc3ZnPgo=" alt="Bronze" /></a>
</p>

---------------

<h1 align="center">Conway's Game of Life</h1>
<div align="center">
    <a href="https://scottgriv.github.io/rust-wasm_game_of_life/" target="_blank">
        <img src="./docs/images/banner.png" style="width: 50%;"/>
    </a>
    <br>
    <i>Rust teams up with WebAssembly.</i>
</div>
<br>

A web-based implementation of **Conway's Game of Life** using `Rust` and `WebAssembly`.

**The Game of Life**, also known simply as **Life**, is a cellular automaton devised by the British mathematician *John Horton Conway* in 1970. It is a zero-player game, meaning that its evolution is determined by its initial state, requiring no further input. One interacts with the **Game of Life** by creating an initial configuration and observing how it evolves. It is *Turing complete* and can simulate a universal constructor or any other *Turing machine*.

 - Visit a demo of the application [here](https://scottgriv.github.io/rust-wasm_game_of_life/).
 
---------------

## Table of Contents

- [Features](#features)
- [Getting Started](#getting-started)
    - [Dependencies](#dependencies)
    - [Installation](#installation)
    - [Usage](#usage)
- [What's Inside?](#whats-inside)
- [Resources](#resources)
- [License](#license)
- [Credits](#credits)

## Features

- **Rust Implementation:** The core logic of the game is implemented in Rust, leveraging its performance and safety features.
- **WebAssembly:** The Rust code is compiled to WebAssembly, enabling it to run efficiently in the browser.
- **Interactive Visualization:** The game grid is rendered in the browser using HTML and JavaScript, allowing users to see the simulation in real-time.

## Getting Started

### Dependencies

1. Install Rust using the recommended method, rustup:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Install wasm-pack using Cargo (the Rust package manager):
```bash
cargo install wasm-pack
```

3. Ensure you have Node.js and npm installed. 
You can download them from [Node.js](https://nodejs.org/).

4. Install http-server globally using npm:
```bash
npm install -g http-server
```

### Installation

1. Clone the Repository:
```bash
git clone https://github.com/scottgriv/rust-wasm_game_of_life
```

2. Change to the Directory:
```bash
cd rust-wasm_game_of_life
```

3. Add Cargo to PATH:
```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

4. Source the Shell Configuration:
```bash
source ~/.zshrc  
```

5. Remove Existing `pkg` Directories (if they exist):
```bash
rm -rf pkg www/pkg
```

6. Build the Project package folder and files with `wasm-pack`:
```bash
wasm-pack build --target web
```

7. Move Generated `pkg` folder to `www`:
```bash
mv pkg www/
```

8. Remove optional Auto-Generated files (if you need them for your repository):
```bash
rm www/pkg/README.md www/pkg/.gitignore
```

9. Set up the Web Server:
```bash
cd www
http-server
```

## Usage

- Open your web browser and navigate to:
```bash
http://localhost:8080
```

<div align="center">
    <a href="https://scottgriv.github.io/rust-wasm_game_of_life/" target="_blank">
        <img src="./docs/images/demo.gif" style="width: 50%;"/>
    </a>
    <br>
    <i>Web output, cellular automaton in a zero-player game.</i>
</div>

## What's Inside?

```bash
rust-wasm_game_of_life/ # Root directory
├── .github/ # GitHub folder
│   └── workflows/ # GitHub Actions folder
│       └── gh-pages.yml # GitHub Pages workflow
├── src/ # Source code
│   └── lib.rs # Rust code for the game logic
├── www/ # Web assets
│   ├── index.html # HTML file
│   ├── index.js # JavaScript file
│   └── pkg/ # WebAssembly package
│       ├── package.json # JSON package file
│       ├── rust_wasm_game_of_life.js
│       ├── rust_wasm_game_of_life_bg.wasm
│       ├── rust_wasm_game_of_life_bg.wasm.d.ts
│       └── rust_wasm_game_of_life.d.ts
├── target/ # Generated
├── Cargo.toml # Dependencies
├── .gitignore # git ignore file
├── LICENSE # License file
└── README.md # This file
```

## Resources

- [Rust](https://www.rust-lang.org/)
- [WebAssembly](https://webassembly.org/)
- [Rust and WebAssembly](https://rustwasm.github.io/book/introduction.html)
- [Conway's Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life)
- [Play Conway's Game of Life](https://playgameoflife.com/)

## License

This project is released under the terms of **The Unlicense**, which allows you to use, modify, and distribute the code as you see fit. 
- [The Unlicense](https://choosealicense.com/licenses/unlicense/) removes traditional copyright restrictions, giving you the freedom to use the code in any way you choose.
- For more details, see the [LICENSE](LICENSE) file in this repository.

## Credits

**Author:** [Scott Grivner](https://github.com/scottgriv) <br>
**Email:** [scott.grivner@gmail.com](mailto:scott.grivner@gmail.com) <br>
**Website:** [scottgrivner.dev](https://www.scottgrivner.dev) <br>
**Reference:** [Main Branch](https://github.com/scottgriv/rust-wasm_game_of_life) <br>

---------------

<div align="center">
    <a href="https://scottgrivner.dev" target="_blank">
        <img src="./docs/images/footer.png" width="100" height="100"/>
    </a>
</div>

<!-- End README -->
