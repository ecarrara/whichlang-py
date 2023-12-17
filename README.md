# whichlang-py

Python bindings for [Whichlang](https://github.com/quickwit-oss/whichlang), a
natural language detection library library written in Rust.

## Installation

Install like any other python package:

```bash
pip install whichlang-py
```

## Usage


```python
from whichlang_py import detect_language

detect_language(
  "A neblina enconbre o Cristo e a lagoa se ilumina" \
  "com edifícios de cabeça para baixo e refletores do Jockey Club")
```
