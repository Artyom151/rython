# rython — план развития

## Фаза 1: Язык (базовый, сделан)
- [x] Транспиляция Python → Rust (переменные, вызовы, while, if/else)
- [x] Runtime: Value, операции, print, сравнения
- [x] Математика: math, time
- [x] PyQt6 FFI (C++ wrapper)
- [x] Многофайловые проекты (import)
- [x] Система wrapper'ов (wrappers/)
- [x] rip — пакетный менеджер
- [x] rython.sh — установка

## Фаза 2: Язык — до полного Python ✅
- [x] `for i in range(n)` — range() в транспайлере + Rust range
- [x] list/tuple/dict/set методы (append, pop, sort, reverse, insert, remove, clear, extend, count, keys, values, items, get, update, setdefault, add, discard)
- [x] str методы (split, join, strip, replace, upper, lower, startswith, endswith, find, rfind, isdigit, isalpha, isalnum, isspace, capitalize, zfill, center, ljust, rjust, partition, rpartition)
- [x] class + наследование + super() (ClassDef в transpiler)
- [x] try/except → catch_unwind (Runtime паника)
- [x] lambda → Rust move closures
- [x] f-strings (JoinedStr + format!)
- [x] async/await (синтаксис)
- [x] isinstance, hasattr (через type_name), abs, min, max, sum, type
- [x] import с произвольным путём (rython_packages + dotted paths)
- [x] `examples/phase2.py` вывод идентичен CPython (кроме адресов памяти)

## Фаза 3: FFI wrapper'ы (C/C++ библиотеки) ✅
- [x] **libtorch (PyTorch)** — тензоры, autograd, nn.Module
- [x] **NumPy C API** — ndarray, ufuncs (pure C++, no deps)
- [x] **SDL2** — окна, ввод, звук
- [x] **OpenGL/Vulkan** — графика (GLX + Vulkan)
- [x] **libcurl** — HTTP
- [x] **SQLite** — базы данных
- [x] **libgit2** — git
- [x] **GTK4 / LVGL** — GUI (GTK4 + LVGL embedded)
- [x] **CUDA** — GPU вычисления (CUDA runtime + cuBLAS)
- [x] **libav (FFmpeg)** — видео (decode, thumbnail, metadata)
- [x] **libpng / libjpeg / libwebp** — изображения (load/save/resize)
- [x] **harfbuzz / freetype** — шрифты (glyph rendering + shaping)

## Фаза 4: Инструменты ✅
- [x] rython fmt
- [x] rython repl
- [x] rython test
- [x] LSP (автодополнение, hover, диагностика)

## Фаза 5: Оптимизация (в процессе)
- [x] Система типов в транспиляторе (Type enum, infer_type, type_map)
- [ ] Типизация: убрать Value где тип известен (генерация Rust-типов, частично)
- [ ] JIT (через LLVM или libgccjit)
- [ ] AOT в нативный код без Rust

## Фаза 6: Динамическое выполнение
- [x] exec() — выполнение Python-кода (compile-time для строк, runtime через python3)
- [x] eval() — вычисление выражений
- [x] exec_runtime / eval_runtime в stdlib

## Структура
```
rython/
├── src/              # Ядро (Rust)
│   ├── wrappers/     # C/C++ wrapper'ы
│   └── ...
├── test/             # Тесты и демо (аналог src/test/ в Java)
│   └── gui/          # PyQt6 GUI
├── scripts/          # Скрипты
└── docs/             # Документация
```
