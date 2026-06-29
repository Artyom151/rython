#!/usr/bin/env bash
set -e

RYTHON_VERSION="v0.1.1"
REPO="Artyom151/rython"

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
BOLD='\033[1m'
NC='\033[0m'

info()  { echo -e "${CYAN}→${NC} $1"; }
ok()    { echo -e "${GREEN}✓${NC} $1"; }
warn()  { echo -e "${YELLOW} WARN ${NC} $1"; }
err()   { echo -e "${RED} ERR ${NC} $1"; }

find_bin_dir() {
    for d in "$HOME/.local/bin" "$HOME/.cargo/bin" "$HOME/bin"; do
        mkdir -p "$d" 2>/dev/null
        if [ -w "$d" ]; then
            echo "$d"
            return
        fi
    done
    for d in /usr/local/bin /usr/bin; do
        if [ -w "$d" ]; then
            echo "$d"
            return
        fi
    done
    echo ""
}

install_binaries() {
    local bin_dir
    bin_dir=$(find_bin_dir)
    if [ -z "$bin_dir" ]; then
        err "No writable directory found in PATH."
        err "Manually: cp target/release/{rython,rip} ~/.local/bin/"
        exit 1
    fi

    if [ -f target/release/rython ]; then
        cp target/release/rython "$bin_dir/rython"
        chmod +x "$bin_dir/rython"
        ok "rython → $bin_dir/rython"
    else
        err "Binary not found. Run '$0 build' first."
        exit 1
    fi

    if [ -f target/release/rip ]; then
        cp target/release/rip "$bin_dir/rip"
        chmod +x "$bin_dir/rip"
        ok "rip → $bin_dir/rip"
    fi

    local in_path=false
    if echo "$PATH" | grep -q "$bin_dir"; then
        in_path=true
    fi

    local fish_cfg="$HOME/.config/fish/config.fish"
    local fish_has=false
    if command -v fish &>/dev/null && [ -f "$fish_cfg" ] && grep -q "$bin_dir" "$fish_cfg" 2>/dev/null; then
        fish_has=true
    fi

    if $in_path && $fish_has; then
        ok "ready to use!"
        return
    fi

    if ! $in_path; then
        local rc_files=("$HOME/.bashrc" "$HOME/.zshrc" "$HOME/.profile" "$HOME/.bash_profile")
        local added=false
        for rc in "${rc_files[@]}"; do
            if [ -f "$rc" ] && ! grep -q "$bin_dir" "$rc" 2>/dev/null; then
                echo "" >> "$rc"
                echo "# rython" >> "$rc"
                echo "export PATH=\"\$PATH:$bin_dir\"" >> "$rc"
                ok "Added $bin_dir to PATH in $rc"
                info "Restart your shell or run: source $rc"
                added=true
                break
            fi
        done
        if ! $added; then
            warn "Could not auto-configure PATH. Add manually: export PATH=\"\$PATH:$bin_dir\""
        fi
    fi

    if ! $fish_has; then
        mkdir -p "$HOME/.config/fish" 2>/dev/null
        echo "" >> "$fish_cfg"
        echo "# rython" >> "$fish_cfg"
        echo "set -gx PATH \$PATH $bin_dir" >> "$fish_cfg"
        ok "Added $bin_dir to fish PATH in $fish_cfg"
        info "Restart fish or run: source $fish_cfg"
    fi
}

check_deps() {
    if ! command -v rustc &>/dev/null; then
        err "Rust is not installed. Run '$0 install-deps' first."
        exit 1
    fi
}

check() {
    info "Checking for updates..."
    local latest
    latest=$(curl -sSL "https://api.github.com/repos/$REPO/releases/latest" 2>/dev/null \
        | python3 -c "import json,sys; d=json.load(sys.stdin); print(d.get('tag_name',''))" 2>/dev/null || true)

    if [ -z "$latest" ]; then
        warn "Could not reach GitHub API."
        info "Current version: $RYTHON_VERSION"
        return
    fi

    info "Current: $RYTHON_VERSION"
    info "Latest:  ${latest}"

    if [ "$latest" != "$RYTHON_VERSION" ] && [ -n "$latest" ]; then
        warn "New version available: $latest"
        info "Run '$0 update' to upgrade."
    else
        ok "Up to date."
    fi
}

update() {
    info "Updating rython from $REPO..."
    if ! command -v git &>/dev/null; then
        err "git is required."
        exit 1
    fi

    local tmpdir
    tmpdir=$(mktemp -d)
    pushd "$tmpdir" >/dev/null
    git clone --depth 1 "https://github.com/$REPO.git" rython 2>/dev/null || {
        err "Clone failed."
        popd >/dev/null
        rm -rf "$tmpdir"
        exit 1
    }
    cd rython
    info "Building..."
    cargo build --release 2>&1 | tail -3
    popd >/dev/null

    local src="$tmpdir/rython/target/release/rython"
    local rip="$tmpdir/rython/target/release/rip"
    local script="$tmpdir/rython/rython.sh"

    if [ -f "$src" ]; then
        local bin_dir; bin_dir=$(find_bin_dir)
        cp "$src" "$bin_dir/rython"
        chmod +x "$bin_dir/rython"
        ok "rython updated."
    fi
    if [ -f "$rip" ]; then
        local bin_dir; bin_dir=$(find_bin_dir)
        cp "$rip" "$bin_dir/rip"
        chmod +x "$bin_dir/rip"
        ok "rip updated."
    fi
    if [ -f "$script" ]; then
        chmod +x "$script"
        info "rython.sh saved at $script"
    fi

    rm -rf "$tmpdir"
    ok "Update complete."
}

install_deps() {
    info "Installing system dependencies..."

    if ! command -v rustc &>/dev/null; then
        info "Installing Rust..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source "$HOME/.cargo/env"
        ok "Rust installed."
    else
        ok "Rust $(rustc --version)"
    fi

    for cmd in curl python3 tar unzip; do
        if ! command -v "$cmd" &>/dev/null; then
            case $(uname -s) in
                Linux)
                    if command -v apt-get &>/dev/null; then
                        sudo apt-get install -y "$cmd" >/dev/null 2>&1
                    elif command -v pacman &>/dev/null; then
                        sudo pacman -S --noconfirm "$cmd" >/dev/null 2>&1
                    elif command -v dnf &>/dev/null; then
                        sudo dnf install -y "$cmd" >/dev/null 2>&1
                    fi
                    ;;
                Darwin)
                    if command -v brew &>/dev/null; then
                        brew install "$cmd" >/dev/null 2>&1
                    fi
                    ;;
            esac
            ok "$cmd"
        fi
    done

    ok "All dependencies installed."
}

install() {
    check_deps

    local bin_dir
    bin_dir=$(find_bin_dir)
    if [ -z "$bin_dir" ]; then
        err "No writable directory in PATH."
        err "Create ~/.local/bin and add it to PATH, then retry."
        exit 1
    fi

    info "Building rython (release)..."
    cargo build --release 2>&1 | tail -3
    ok "Build complete."

    install_binaries
    ok "rython and rip are now in PATH. Use: rython help  |  rip help"
}

build() {
    check_deps
    info "Building rython (release)..."
    cargo build --release 2>&1 | tail -3
    ok "Build complete: target/release/{rython,rip}"
}

version() {
    echo "rython v$RYTHON_VERSION"
    command -v rustc &>/dev/null && echo "rustc $(rustc --version | cut -d' ' -f2)"
}

case "${1:-help}" in
    check)
        check ;;
    update|upgrade)
        update ;;
    install-deps|deps)
        install_deps ;;
    install)
        install ;;
    build)
        build ;;
    version|--version|-v)
        version ;;
    help|--help|-h|*)
        echo -e "${BOLD}rython.sh — rython management${NC}"
        echo ""
        echo "USAGE:"
        echo "  $0 install       Build release binaries and copy to PATH"
        echo "  $0 install-deps  Install Rust + system tools"
        echo "  $0 build         Build release binaries (no install)"
        echo "  $0 check         Check for updates"
        echo "  $0 update        Fetch, build, and install latest version"
        echo "  $0 version       Show version info"
        echo "  $0 help          Show this help"
        echo ""
        echo "EXAMPLES:"
        echo "  $0 install-deps"
        echo "  $0 install"
        echo "  $0 check"
        echo "  $0 update"
        ;;
esac
