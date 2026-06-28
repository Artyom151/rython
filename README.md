<p align="center">
  <img src="logo.svg" alt="rython logo" width="200">
</p>

<h1 align="center">rython</h1>

<p align="center">
  <strong>Python to Rust transpiler</strong><br>
  Write Python, ship native binaries.
</p>

<p align="center">
  <a href="#features">Features</a> •
  <a href="#quick-start">Quick Start</a> •
  <a href="#usage">Usage</a> •
  <a href="#examples">Examples</a> •
  <a href="#how-it-works">How It Works</a> •
  <a href="#ffi-wrappers">FFI Wrappers</a> •
  <a href="#building">Building</a>
</p>

---

## Overview

rython is a Python-to-Rust transpiler that converts Python source code into idiomatic Rust, compiles it, and produces a standalone native binary. It supports a large subset of Python 3 syntax and standard library modules, plus a system of C/C++ FFI wrappers for accessing native libraries directly from Python code.

Instead of interpreting Python or running a VM, rython translates your Python code to Rust at build time. The result is a compiled binary with no runtime dependency on Python.

---

## Features

- **Full Python-to-Rust transpilation**: variables, functions, classes, inheritance, control flow, and comprehensions
- **Rich runtime library**: 40+ methods on Value (list, dict, str, set), builtins (isinstance, hasattr, abs, min, max, sum, type, len, any, all, enumerate, zip, reversed, sorted)
- **Exception handling**: try/except compiled to Rust catch_unwind
- **Lambda and closures**: Python lambdas transpiled to Rust move closures
- **Async/await syntax**: parsed and transpiled (synchronous evaluation)
- **F-strings**: transpiled to Rust format! macro
- **Import system**: stdlib modules, local Python files, dotted paths, rython_packages support
- **Standard library modules**: math, sys, os, json, time, re, random, collections, itertools, functools, pathlib, datetime, typing, abc, urllib, and more
- **15 C/C++ FFI wrappers**: PyTorch, NumPy, SDL2, OpenGL, Vulkan, libcurl, SQLite, libgit2, GTK4, LVGL, CUDA/cuBLAS, FFmpeg, image (PNG/JPEG/WebP), font rendering (FreeType + harfbuzz)
- **CLI tools**: formatter (rython fmt), REPL (rython repl), test runner (rython test), LSP server (rython lsp)
- **Package manager**: rip install for dependency management
- **Zero runtime dependencies**: the output binary is a standalone executable

---

## Quick Start

### Install

```bash
git clone https://github.com/Artyom151/rython.git
cd rython
cargo build --release
cp target/release/rython ~/.local/bin/
```

### Transpile and run a Python file

```bash
rython hello.py
```

### Transpile with verbose output (show tokens, generated Rust, compilation log, execution)

```bash
rython hello.py --verbose
```

### Generate Rust source only

```bash
rython hello.py --output hello.rs
```

---

## Usage

### Transpiler

The main command transpiles a Python file to Rust, compiles it, and runs the resulting binary.

```bash
rython <file.py>           # transpile, compile, run
rython <file.py> --verbose # show tokens, generated code, compilation
rython <file.py> --output <file>  # save generated Rust source
rython <file.py> --output <file> --run  # save and run
```

### Formatter

```bash
rython fmt <file.py>                # format in-place
rython fmt --check <file.py>        # check without modifying
rython fmt --diff <file.py>         # show diff
```

### REPL

```bash
rython repl
```

An interactive multi-line REPL. Each statement is transpiled, compiled via rustc, and executed. Type `exit()` to quit.

### Test Runner

```bash
rython test <file.py>
```

Discovers all `def test_*` functions, compiles each into a separate binary, runs them, and reports results.

### LSP Server

```bash
rython lsp
```

A Language Server Protocol server over stdin/stdout. Supports:

- Document synchronization (didOpen, didChange, didClose)
- Completions (keywords, builtins, standard library modules)
- Hover documentation (type information, docstrings)
- Diagnostics (parser errors via catch_unwind)
- Go-to-definition

---

## Examples

### Hello World

```python
print("Hello, World!")
print(42)
```

### Fibonacci

```python
def fib(n):
    a, b = 0, 1
    while a < n:
        print(a)
        a, b = b, a + b

fib(100)
```

### Classes and Inheritance

```python
class Point:
    def __init__(self, x, y):
        self.x = x
        self.y = y

    def distance(self, other):
        dx = self.x - other.x
        dy = self.y - other.y
        return (dx * dx + dy * dy) ** 0.5

p1 = Point(1, 2)
p2 = Point(4, 6)
print(p1.distance(p2))
```

### Exception Handling

```python
try:
    x = 1 / 0
except:
    print("caught exception")
```

### Lambda and Higher-Order Functions

```python
f = lambda x, y: x + y
print(f(10, 20))

result = sorted([3, 1, 4, 1, 5])
print(result)

pairs = [(1, 'one'), (3, 'three'), (2, 'two')]
for k, v in enumerate(["a", "b", "c"]):
    print(k, v)
```

A comprehensive test file covering 44 Python features can be found at `examples/phase2.py`. Its output is identical to CPython (except for memory addresses in iterator representations).

---

## How It Works

rython operates in three stages:

### 1. Tokenization and Parsing

The Python source is tokenized by `src/lexer.rs` into a stream of tokens, then parsed by `src/parser.rs` into an abstract syntax tree (AST). The parser supports the full Python grammar needed for transpilation: function definitions, class definitions, imports, control flow, comprehensions, lambda, try/except, with statements, async/await, f-strings, and all expression types.

### 2. Transpilation

`src/transpiler.rs` walks the AST and generates equivalent Rust code. The transpiler maintains state for variable usage, imported modules, class methods, and Qt/SDL/OpenGL object tracking. Key design decisions:

- All Python values are represented as the `Value` enum in `src/runtime.rs` — a tagged union supporting Int, Float, Bool, Str, List, Tuple, Dict, Set, Slice, Range, Bytes, None, and Ptr (for opaque C++ objects)
- Methods on built-in types are dispatched through `VALUE_METHODS`, `MUTABLE_VALUE_METHODS`, and `VEC_ARGS_VALUE_METHODS` tables
- `try/except` compiles to `std::panic::catch_unwind`
- Lambda compiles to Rust closures
- F-strings compile to the `format!` macro

### 3. Compilation

The generated Rust source is compiled with `rustc`. C++ FFI wrappers that are referenced by the Python code are compiled alongside as `.o` files and linked. The result is a standalone native binary.

The Rust output includes:

- `src/runtime.rs` — the `Value` type and all its methods
- `src/stdlib.rs` — pure-Rust implementations of Python standard library modules
- `src/wrappers.rs` — Rust FFI declarations for C++ wrapper functions
- Transpiled user code (definitions and statements)

---

## FFI Wrappers

rython includes a system of C++ FFI wrappers for accessing native libraries. Each wrapper in `src/wrappers/` is a `.cpp` file that exposes C-compatible functions. The corresponding Rust declarations are in `src/wrappers.rs`.

Wrappers are automatically linked when the transpiled Rust code contains `use wrappers::<name>::` or `crate::wrappers::<name>::`. Missing system libraries produce a warning (visible only with `--verbose`) — the compilation continues without the missing wrapper.

### Available Wrappers

| Wrapper | Library | pkg-config |
|---------|---------|------------|
| qt6 | Qt6 (Widgets, Core, Gui) | Qt6Widgets Qt6Core Qt6Gui |
| sqlite3 | SQLite | sqlite3 |
| sdl2 | SDL2 + SDL2_image, ttf, mixer | sdl2 SDL2_image SDL2_ttf SDL2_mixer |
| curl | libcurl | libcurl |
| image | libpng, libjpeg, libwebp | libpng libjpeg libwebp |
| opengl | OpenGL + X11 | x11 gl |
| git | libgit2 | libgit2 |
| gtk4 | GTK4 | gtk4 |
| ffmpeg | FFmpeg (avformat, avcodec, avutil, swscale) | libavformat libavcodec libavutil libswscale |
| font | FreeType + harfbuzz | freetype2 harfbuzz |
| vulkan | Vulkan | vulkan |
| torch | libtorch (PyTorch C++ API) | libtorch |
| numpy | NumPy C API (self-contained C++) | (none) |
| lvgl | LVGL | lvgl |
| cuda | CUDA + cuBLAS | (none, runtime check) |

---

## Project Structure

```
rython/
├── src/
│   ├── main.rs              # CLI entry point, compile_rust, command dispatch
│   ├── lib.rs                # Library root with submodule declarations
│   ├── lexer.rs              # Python tokenizer
│   ├── parser.rs             # Python parser (AST construction)
│   ├── transpiler.rs         # Python-to-Rust code generator
│   ├── runtime.rs            # Value type and all runtime operations
│   ├── stdlib.rs             # Standard library module implementations
│   ├── wrappers.rs           # Rust FFI declarations for C++ wrappers
│   ├── formatter.rs          # Python code formatter
│   ├── lsp.rs                # LSP server implementation
│   ├── ast.rs                # AST type definitions
│   ├── bin/
│   │   └── rip.rs            # Package manager (rip)
│   └── wrappers/             # C++ FFI wrapper implementations
│       ├── qt6.cpp
│       ├── sqlite3.cpp
│       ├── sdl2.cpp
│       ├── curl.cpp
│       ├── image.cpp
│       ├── opengl.cpp
│       ├── git.cpp
│       ├── gtk4.cpp
│       ├── ffmpeg.cpp
│       ├── font.cpp
│       ├── vulkan.cpp
│       ├── torch.cpp
│       ├── numpy.cpp
│       ├── lvgl.cpp
│       └── cuda.cpp
├── examples/                # Example Python programs
│   ├── phase2.py            # Comprehensive feature test
│   ├── classes.py
│   ├── fib.py
│   ├── factorial.py
│   ├── bench.py             # Performance benchmark
│   └── ...
├── tests/                   # Test files
├── scripts/                 # Build and utility scripts
├── docs/                    # Documentation
├── rython.sh                # Installation script
├── Cargo.toml
└── README.md
```

---

## Performance

As a natively compiled binary, rython-transpiled programs typically run faster than CPython. The benchmark at `examples/bench.py` demonstrates this:

| Runtime | Time |
|---------|------|
| CPython 3.x | 0.139s |
| rython (native binary) | 0.045s |

Performance gains come from:

- No interpreter overhead
- LLVM optimization via rustc
- Static type dispatch for runtime Value operations
- Inlined arithmetic and string operations

---

## Building

### Prerequisites

- Rust toolchain (rustc, cargo)
- C++ compiler (g++ or clang++)
- pkg-config
- Optional: native libraries for FFI wrappers (see tables above)

### Build from source

```bash
git clone https://github.com/Artyom151/rython.git
cd rython
cargo build --release
```

The binary will be at `target/release/rython`.

### Install

```bash
./rython.sh install
```

This builds the release binary and copies it to `~/.local/bin/rython`.

---

## Limitations

- **async/await is syntax-only**: no actual concurrency is introduced at the Rust level
- **Dict iteration order**: uses BTreeMap, which sorts by key rather than preserving insertion order
- **No dynamic code execution**: eval and exec are not supported (the Python source must be known at transpile time)
- **Some stdlib modules are partial**: only commonly used functions are implemented (e.g., math.sqrt, os.getcwd, json.dumps/loads)
- **FFI wrappers require system libraries**: wrappers for missing libraries are silently skipped (compile without the corresponding feature)

---

## License

MIT License — Copyright (c) 2026 Artyom151. See [LICENSE](LICENSE).
