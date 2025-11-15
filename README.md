# TypeMan
Typing speed test with practice mode in GUI, TUI and CLI

![Rust](https://img.shields.io/badge/Powered%20by-Rust-red)
[![Crates.io](https://img.shields.io/crates/v/typeman.svg)](https://crates.io/crates/typeman)
![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)
![PRs](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)

<br>
<br>
<p float="left">
<img src="screenshots/image-3.png" alt="Alt Text" width="400">
<img src="screenshots/image-1.png" alt="Alt Text" width="400">
</p>

<p float="left">
<img src="screenshots/image-9.png" alt="Alt Text" width="400">
<img src="screenshots/image-10.png" alt="Alt Text" width="400">
</p>
<img src="screenshots/image-11.png" alt="Alt Text" width="700">  
<br>
<br>
<details>
<summary> Additional screenshots</summary>

<br>
<img src="screenshots/image-6.png" alt="Alt Text" width="700">
<br>
<img src="screenshots/image-4.png" alt="Alt Text" width="700">
<br>
<img src="screenshots/image-7.png" alt="Alt Text" width="700">
<br>
<img src="screenshots/image-8.png" alt="Alt Text" width="700">
<br>
<img src="screenshots/image-2.png" alt="Alt Text" width="700">
<br>
<img src="screenshots/image-5.png" alt="Alt Text" width="700">

</details>
<br>


## Installation

### Crates.io:
#### Install the default version (all modes: CLI, GUI, TUI):
    cargo install typeman

> [!NOTE]  
> GUI feature may be too heavy for your needs, in this case follow instructions below

#### You can also install only the modes you want by using `--no-default-features` and specifying features:
- **only TUI**:   
    ```
    cargo install typeman --no-default-features --features tui
    ```
- **only TUI and CLI**:  
    ```
    cargo install typeman --no-default-features --features "tui cli"
    ```
--- 
### Tweaks on installs:

Additional action needed for succesfull install

##### On MacOS:
    cargo install --target x86_64-apple-darwin typeman
##### On Ubuntu (before installing):
    sudo apt-get install libfontconfig1-dev
    sudo apt install libasound2-dev
##### On Fedora (before installing):
    sudo dnf install alsa-lib-devel

---

### From source:
#### 1. clone repo  
    git clone https://github.com/mzums/typeman      
#### 2. enter project
    cd typeman
#### 3. run
    cargo run

## Modes:
- **TUI** (ratatui)
- **GUI** (macroquad)
- **CLI**

## Features:
- multi-language support
- theme selection
- local leaderboard
- saving user interface preferences 
- top words and batch size preferences

## CLI parameters:
- **word number**: number of displayed words
- **top words**: number of top most common english words used to generae test
- **time**: duration of the test in time mode
- **quote**: random quote
- **punctuation**: punctuation in word number and time modes
- **digits**: digits  in word and time modes
- **level**: practice level
- **wikipedia**: wikipedia snippets

## Commands:
- `typeman` - TUI
- `typeman --gui` - GUI
- `typeman --cli` - CLI
    - `typeman --cli -c ./text.txt` - custom file
    - `typeman --cli -q` - random quote
    - `typeman --cli (-t=30) -n=500` - 30s (default) test with random words from 500 most used english words
    - `typeman --cli -w=50 -n=500 -p -d` - 50 random words from 500 most used english words with punctuation and digits
    - `typeman --cli -l` - list all practice levels
    - `typeman --cli -l=1` - practice first level
    - `typeman --cli --wiki` - wikipedia mode

---

### Credits:
- https://github.com/dwyl/quotes  
- https://github.com/JackShannon/1000-most-common-words/blob/master/1000-common-english-words.txt
- Indonesian common words compiled from various online dictionaries and frequency lists
- https://fonts.google.com/specimen/Roboto+Mono?preview.text=Whereas%20recognition%20of%20the%20inherent%20dignity
- Wikipedia

### Special thanks to [piter231](https://github.com/piter231/) for testing!
