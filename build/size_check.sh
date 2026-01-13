#!/bin/bash
# Binary size reporting for Cortex-μKernel

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
cd "$PROJECT_ROOT"

MAX_SIZE_KB=20
MAX_SIZE_BYTES=$((MAX_SIZE_KB * 1024))

# echo_color prints the given message(s) to stdout using the ANSI color code specified by the first argument.
echo_color() {
    local color=$1
    shift
    echo -e "\033[${color}m$*\033[0m"
}

echo_color "35" "=========================================="
echo_color "35" "Cortex-μKernel Binary Size Report"
echo_color "35" "=========================================="
echo ""

targets=(
    "x86_64-cortex-kernel"
    "aarch64-cortex-kernel"
    "armv7-cortex-kernel"
    "riscv64-cortex-kernel"
)

total_size=0
pass_count=0
fail_count=0

for target in "${targets[@]}"; do
    binary="target/${target}/release/kernel"

    if [ -f "$binary" ]; then
        size_bytes=$(stat -f%z "$binary" 2>/dev/null || stat -c%s "$binary" 2>/dev/null)
        size_kb=$((size_bytes / 1024))
        total_size=$((total_size + size_bytes))

        echo_color "36" "$target:"
        echo "  Size: ${size_kb} KB (${size_bytes} bytes)"
        echo "  Limit: ${MAX_SIZE_KB} KB (${MAX_SIZE_BYTES} bytes)"

        if [ $size_bytes -le $MAX_SIZE_BYTES ]; then
            echo_color "32" "  ✓ WITHIN LIMIT"
            ((pass_count++))
        else
            overflow=$((size_bytes - MAX_SIZE_BYTES))
            echo_color "31" "  ✗ EXCEEDS LIMIT by ${overflow} bytes"
            ((fail_count++))
        fi
        echo ""
    else
        echo_color "33" "$target: NOT BUILT (run ./build/build.sh all)"
        echo ""
    fi
done

echo_color "35" "=========================================="
echo_color "35" "Summary"
echo_color "35" "=========================================="
echo "Total size: $((total_size / 1024)) KB"
echo ""
echo_color "32" "Passed: $pass_count targets"
if [ $fail_count -gt 0 ]; then
    echo_color "31" "Failed: $fail_count targets"
fi
echo ""

if [ $fail_count -eq 0 ] && [ $pass_count -eq ${#targets[@]} ]; then
    echo_color "32" "✓ All targets within size limits!"
    exit 0
elif [ $fail_count -gt 0 ]; then
    echo_color "31" "✗ Some targets exceed size limits"
    exit 1
else
    echo_color "33" "⚠ Build all targets with: ./build/build.sh all"
    exit 0
fi