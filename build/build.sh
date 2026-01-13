#!/bin/bash
# Multi-platform build orchestrator for Cortex-μKernel

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
cd "$PROJECT_ROOT"

echo_color() {
    local color=$1
    shift
    echo -e "\033[${color}m$*\033[0m"
}

report_size() {
    local binary="$1"
    local target="$2"

    if [ -f "$binary" ]; then
        if command -v size &> /dev/null; then
            echo_color "36" "$target binary size:"
            size -A "$binary" || echo "  (size command not available)"
        else
            local size_bytes
            if command -v stat &> /dev/null; then
                size_bytes=$(stat -f%z "$binary" 2>/dev/null || stat -c%s "$binary" 2>/dev/null)
                local size_kb=$((size_bytes / 1024))
                echo_color "36" "$target binary size: ${size_kb} KB"
            fi
        fi
    else
        echo_color "31" "Error: Binary not found at $binary"
        return 1
    fi
}

build_x86_64() {
    echo_color "32" "Building x86_64 target..."
    cargo build --target x86_64-cortex-kernel --release --bin kernel
    report_size "target/x86_64-cortex-kernel/release/kernel" "x86_64"
    echo_color "32" "x86_64 build complete."
}

build_arm64() {
    echo_color "32" "Building ARM64 (AArch64) target..."
    cargo build --target aarch64-cortex-kernel --release --bin kernel
    report_size "target/aarch64-cortex-kernel/release/kernel" "ARM64"
    echo_color "32" "ARM64 build complete."
}

build_arm32() {
    echo_color "32" "Building ARM32 (ARMv7) target..."
    cargo build --target armv7-cortex-kernel --release --bin kernel
    report_size "target/armv7-cortex-kernel/release/kernel" "ARM32"
    echo_color "32" "ARM32 build complete."
}

build_riscv() {
    echo_color "32" "Building RISC-V 64 target..."
    cargo build --target riscv64-cortex-kernel --release --bin kernel
    report_size "target/riscv64-cortex-kernel/release/kernel" "RISC-V64"
    echo_color "32" "RISC-V build complete."
}

build_all() {
    echo_color "35" "========================================="
    echo_color "35" "Building Cortex-μKernel (all targets)"
    echo_color "35" "========================================="
    echo ""

    build_x86_64
    echo ""
    build_arm64
    echo ""
    build_arm32
    echo ""
    build_riscv
    echo ""

    echo_color "32" "========================================="
    echo_color "32" "All targets built successfully!"
    echo_color "32" "========================================="
}

case "${1:-all}" in
    x86_64|x86_64-cortex-kernel)
        build_x86_64
        ;;
    arm64|aarch64|aarch64-cortex-kernel)
        build_arm64
        ;;
    arm32|armv7|armv7-cortex-kernel)
        build_arm32
        ;;
    riscv|riscv64|riscv64-cortex-kernel)
        build_riscv
        ;;
    all)
        build_all
        ;;
    clean)
        echo_color "33" "Cleaning build artifacts..."
        cargo clean
        echo_color "32" "Clean complete."
        ;;
    help|--help|-h)
        echo "Cortex-μKernel Build System"
        echo ""
        echo "Usage: ./build.sh [TARGET]"
        echo ""
        echo "Targets:"
        echo "  x86_64    - Build x86_64 target"
        echo "  arm64     - Build AArch64 target"
        echo "  arm32     - Build ARMv7 target"
        echo "  riscv     - Build RISC-V 64 target"
        echo "  all       - Build all targets (default)"
        echo "  clean     - Clean build artifacts"
        echo "  help      - Show this help message"
        echo ""
        echo "Examples:"
        echo "  ./build.sh x86_64"
        echo "  ./build.sh all"
        echo "  ./build.sh clean"
        ;;
    *)
        echo_color "31" "Unknown target: $1"
        echo ""
        ./build.sh help
        exit 1
        ;;
esac
