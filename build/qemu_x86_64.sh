#!/bin/bash
# QEMU runner for x86_64 target

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
cd "$PROJECT_ROOT"

KERNEL_BINARY="target/x86_64-cortex-kernel/release/kernel"

if [ ! -f "$KERNEL_BINARY" ]; then
    echo "Error: Kernel binary not found at $KERNEL_BINARY"
    echo "Please build the kernel first: ./build/build.sh x86_64"
    exit 1
fi

if ! command -v qemu-system-x86_64 &> /dev/null; then
    echo "Error: qemu-system-x86_64 not found. Please install QEMU."
    echo ""
    echo "Installation:"
    echo "  Ubuntu/Debian: sudo apt-get install qemu-system-x86"
    echo "  macOS:         brew install qemu"
    echo "  Fedora:        sudo dnf install qemu-system-x86"
    exit 1
fi

echo "Starting Cortex-μKernel (x86_64) in QEMU..."
echo "Kernel: $KERNEL_BINARY"
echo ""

qemu-system-x86_64 \
    -kernel "$KERNEL_BINARY" \
    -serial stdio \
    -nographic \
    -m 256M \
    -s -S
