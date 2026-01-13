#!/bin/bash
# QEMU runner for ARM64 (AArch64) target

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
cd "$PROJECT_ROOT"

KERNEL_BINARY="target/aarch64-cortex-kernel/release/kernel"

if [ ! -f "$KERNEL_BINARY" ]; then
    echo "Error: Kernel binary not found at $KERNEL_BINARY"
    echo "Please build the kernel first: ./build/build.sh arm64"
    exit 1
fi

if ! command -v qemu-system-aarch64 &> /dev/null; then
    echo "Error: qemu-system-aarch64 not found. Please install QEMU."
    echo ""
    echo "Installation:"
    echo "  Ubuntu/Debian: sudo apt-get install qemu-system-arm"
    echo "  macOS:         brew install qemu"
    echo "  Fedora:        sudo dnf install qemu-system-aarch64"
    exit 1
fi

echo "Starting Cortex-μKernel (ARM64) in QEMU..."
echo "Kernel: $KERNEL_BINARY"
echo ""

qemu-system-aarch64 \
    -machine virt \
    -kernel "$KERNEL_BINARY" \
    -serial stdio \
    -nographic \
    -m 256M \
    -cpu cortex-a57 \
    -s -S
