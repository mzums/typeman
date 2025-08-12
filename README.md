# TypeMan
Cli tool for typing learning and tests

![alt text](image.png)
![alt text](image-1.png)

## Modes:
- **TUI** (ratatui)
- **GUI** (macroquad)
- **CLI**

## CLI parameters:
- **word number**: number of displayed words
- **top words**: number of top most common english words used to generae test
- **time**: duration of the test in time mode
- **punctuation**: punctuation in word number and time modes
- **digits**: digits  in word and time modes

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

---

### Credits:
- https://github.com/dwyl/quotes  
- https://github.com/JackShannon/1000-most-common-words/blob/master/1000-common-english-words.txt
- https://fonts.google.com/specimen/Roboto+Mono?preview.text=Whereas%20recognition%20of%20the%20inherent%20dignity