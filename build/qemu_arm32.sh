#!/bin/bash
# QEMU runner for ARM32 (ARMv7) target

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
cd "$PROJECT_ROOT"

KERNEL_BINARY="target/armv7-cortex-kernel/release/kernel"

if [ ! -f "$KERNEL_BINARY" ]; then
    echo "Error: Kernel binary not found at $KERNEL_BINARY"
    echo "Please build the kernel first: ./build/build.sh arm32"
    exit 1
fi

if ! command -v qemu-system-arm &> /dev/null; then
    echo "Error: qemu-system-arm not found. Please install QEMU."
    echo ""
    echo "Installation:"
    echo "  Ubuntu/Debian: sudo apt-get install qemu-system-arm"
    echo "  macOS:         brew install qemu"
    echo "  Fedora:        sudo dnf install qemu-system-arm"
    exit 1
fi

echo "Starting Cortex-μKernel (ARM32) in QEMU..."
echo "Kernel: $KERNEL_BINARY"
echo ""

qemu-system-arm \
    -machine mps2-an505 \
    -kernel "$KERNEL_BINARY" \
    -serial stdio \
    -nographic \
    -m 256M \
    -cpu cortex-m4 \
    -s -S
