# Changelog

## [v0.1.2] - 2026-06-29

### Added
- **rip install (no args)** — install dependencies from `rython.json` manifest
- **rip add \<package\>** — install package(s) and save to `rython.json`
- **rip build \<file.py\>** — compile project to standalone binary with `rustc -O`
- **Parallel package installation** — up to 4 worker threads install packages concurrently with shared dependency graph
- **Wheel fallback** — when sdist has no `.py` files, automatically download and install `.whl` for C-extensions (pydantic-core, multidict, etc.)
- **`.pyc` cache** — after installation, compile all `.py` files to `__pycache__/*.pyc` for 2-3× faster startup
- **Network retry** — curl with `--retry 2 --connect-timeout 15` and exponential backoff (400ms, 800ms, 1200ms) for PyPI 500 errors
- **Manifest persistence** — `rython.json` stores dependencies; `rip uninstall` removes from manifest

### Changed
- **`install_package` → `install_parallel`** — batch install now uses thread pool instead of sequential recursion
- **`fetch_url` / `fetch_bytes`** — added retry loop with delay instead of single curl attempt

### Fixed
- **C-extension ModuleNotFoundError** — wheel fallback resolves native extensions that sdist cannot provide
- **Duplicate dependency resolution** — shared `Arc<Mutex<HashSet>>` prevents resolving same package twice in parallel installs

## [v0.1.1] - 2026-06-29

### Added
- **`super()` support** — single inheritance shortcut transpiles to `self_.clone()`
- **`@property` decorator** — getter evaluated eagerly in constructor
- **Walrus operator `:=`** — `NamedExpr` in AST with parser backtracking, transpiles to block expression
- **`raise Exception("msg")`** — panic-based exception with automatic name suffix recognition
- **`yield` generators** — functions with `yield` accumulate to `Vec<Value>` and return `Value::List`
- **`dataclasses`** — auto-generate `__init__`, `__repr__`, `__eq__` from annotated fields
- **`__name__` builtin constant** — compiles to `Value::Str("__main__")`
- **`if __name__ == "__main__"` skip** — body transpiled unconditionally, wrapper `if` dropped
- **`from math import sqrt, sin, pi`** — stdlib module imports work correctly again
- **`round()` builtin** — transpiles `.to_float().round()` with precision support
- **dict subscript assignment** — `d["key"] = val` now generates `.set_item()`

### Fixed
- **Parser: `elif` panic** — `parse_if` expected `If` token but `elif` recursion sent `Elif`
- **Parser: `is not` double-advance** — `parse_comparison` consumed an extra token after multi-token operators
- **Transpiler: `self_` not found in constructor** — constructor variable was named `obj` but `super()` emitted `self_.clone()`
- **Transpiler: `Qt` undefined** — `from PyQt6.QtCore import Qt` now generates `__qt_value()` function
- **Transpiler: `is_in`, `is_not`, `bitor`, `bitand` reference type** — these methods expect `&Value` but transpiler passed `Value`
- **Transpiler: `used_vars` scope bleeding** — class method bodies shared `used_vars` with the enclosing scope
- **Transpiler: try-body closure return type** — `catch_unwind` closure returned `()` when `return Value::None` forced `Value`
- **Transpiler: if-block semicolons** — `if` without `else` was an expression instead of a statement
- **Transpiler: `floordiv` reference type** — missing `&` before argument
- **Transpiler: float literal formatting** — `Value::Float(10)` instead of `Value::Float(10.0)` for integer-valued floats
- **Runtime: `bitor`/`bitand` for Bool** — panicked on `Value::Bool` operands; now handles boolean `||` and `&&`
- **Runtime: `set_item` missing** — dict/list subscript assignment had no method; added `Value::set_item()`
- **PyQt6 submodule imports** — `from PyQt6.QtCore import Qt` and `from PyQt6.QtGui import QFont` now handled

### Changed
- **Class constructor variable** renamed from `obj` to `self_` for consistency with `super()` transpilation
- **`rython.sh`** updated to version v0.1.1
