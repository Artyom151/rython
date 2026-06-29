#!/usr/bin/env bash
# rython test runner — build & test everything (Java-style `mvn test`)
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
RYTHON="$PROJECT_DIR/target/debug/rython"
RYTHON_REL="./target/debug/rython"

PASS=0
FAIL=0
START=$(date +%s%N)

red()   { echo -e "\033[0;31m$1\033[0m"; }
green() { echo -e "\033[0;32m$1\033[0m"; }
cyan()  { echo -e "\033[0;36m$1\033[0m"; }
bold()  { echo -e "\033[1m$1\033[0m"; }

run_test() {
    local file="$1"
    local mode="${2:-run}"
    local label="$file"
    if [ "$mode" = "test" ]; then
        label="$file (rython test)"
    fi

    set +e
    if [ "$mode" = "test" ]; then
        "$RYTHON" test "$SCRIPT_DIR/$file" > /tmp/rython_test_out.txt 2>&1
        local ec=$?
    else
        "$RYTHON" "$SCRIPT_DIR/$file" > /tmp/rython_test_out.txt 2>&1
        local ec=$?
    fi
    set -e

    if [ $ec -eq 0 ]; then
        green "  PASS  $label"
        PASS=$((PASS + 1))
    else
        red "  FAIL  $label (exit $ec)"
        cat /tmp/rython_test_out.txt
        FAIL=$((FAIL + 1))
    fi
}

# ── Step 1: Build ────────────────────────────────────────────
echo ""
bold "=== Build ==="
cd "$PROJECT_DIR"
cargo build 2>&1 | tail -3
green "  Build OK"

# ── Step 2: Test files (rython test mode) ───────────────────
echo ""
bold "=== Unit tests (rython test) ==="
for f in test_types.py test_eval_exec.py test_eval_exec2.py test_exec.py; do
    if [ -f "$SCRIPT_DIR/$f" ]; then
        run_test "$f" "test"
    fi
done

# ── Step 3: Execution tests (transpile + run) ───────────────
echo ""
bold "=== Execution tests (transpile + run) ==="
for f in \
    hello.py factorial.py fib.py classes.py comprehensions.py \
    with_stmt.py use_math.py use_helper.py use_helper2.py phase2.py \
    test_stdlib.py test_import_from.py test_sys_json.py test_os.py bench.py; do
    if [ -f "$SCRIPT_DIR/$f" ]; then
        run_test "$f" "run"
    fi
done

# ── Step 4: Wrapper checks ──────────────────────────────────
echo ""
bold "=== Wrapper checks ==="
WRAPPERS_DIR="$PROJECT_DIR/src/wrappers"
for wrapper in qt6 sqlite3 sdl2 curl image opengl git gtk4 ffmpeg font vulkan torch numpy lvgl cuda; do
    cpp_file="$WRAPPERS_DIR/$wrapper.cpp"
    if [ -f "$cpp_file" ]; then
        green "  OK   $wrapper.cpp"
        PASS=$((PASS + 1))
    else
        red "  MISS $wrapper.cpp"
        FAIL=$((FAIL + 1))
    fi
done

# ── Step 5: Summary ─────────────────────────────────────────
END=$(date +%s%N)
DURATION_MS=$(( (END - START) / 1000000 ))
MIN=$(( DURATION_MS / 60000 ))
SEC=$(( (DURATION_MS % 60000) / 1000 ))
MS=$(( DURATION_MS % 1000 ))

echo ""
bold "═══════════════════════════════════════════"
if [ $FAIL -eq 0 ]; then
    green "  ALL $PASS TESTS PASSED  (${MIN}m ${SEC}.${MS}s)"
else
    red "  $FAIL / $((PASS + FAIL)) TESTS FAILED  (${MIN}m ${SEC}.${MS}s)"
fi
bold "═══════════════════════════════════════════"
echo ""

exit $FAIL
