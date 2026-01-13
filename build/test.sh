#!/bin/bash
# Comprehensive test suite for Cortex-μKernel

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
cd "$PROJECT_ROOT"

echo_color() {
    local color=$1
    shift
    echo -e "\033[${color}m$*\033[0m"
}

test_count=0
pass_count=0
fail_count=0

run_test() {
    local test_name="$1"
    local test_command="$2"

    ((test_count++))
    echo_color "36" "[$test_count] Running: $test_name"

    if eval "$test_command"; then
        echo_color "32" "    ✓ PASS"
        ((pass_count++))
        return 0
    else
        echo_color "31" "    ✗ FAIL"
        ((fail_count++))
        return 1
    fi
}

echo_color "35" "========================================="
echo_color "35" "Cortex-μKernel Test Suite"
echo_color "35" "========================================="
echo ""

# Unit tests
echo_color "33" "Unit Tests"
echo_color "33" "-----------"
run_test "Verifier library tests" "cargo test --lib verifier --release 2>&1 | grep -q 'test result' && cargo test --lib verifier --release"

if [ -d "verifier" ] && [ -f "verifier/src/lib.rs" ]; then
    echo ""
else
    echo_color "33" "  (Verifier tests skipped - no test code yet)"
fi
echo ""

# Kernel library tests
echo_color "33" "Kernel Library Tests"
echo_color "33" "---------------------"
if [ -d "kernel" ] && [ -f "kernel/src/lib.rs" ]; then
    run_test "Kernel library tests" "cargo test --lib kernel --release 2>&1 | grep -q 'test result' && cargo test --lib kernel --release"
else
    echo_color "33" "  (Kernel library tests skipped - no test code yet)"
fi
echo ""

# Slots library tests
echo_color "33" "Slots Library Tests"
echo_color "33" "--------------------"
if [ -d "slots" ] && [ -f "slots/src/lib.rs" ]; then
    run_test "Slots library tests" "cargo test --lib slots --release 2>&1 | grep -q 'test result' && cargo test --lib slots --release"
else
    echo_color "33" "  (Slots tests skipped - no test code yet)"
fi
echo ""

# Build tests
echo_color "33" "Build Tests"
echo_color "33" "-----------"
run_test "x86_64 release build" "cargo build --target x86_64-cortex-kernel --release --bin kernel 2>&1 | grep -q 'Finished'"
run_test "ARM64 release build" "cargo build --target aarch64-cortex-kernel --release --bin kernel 2>&1 | grep -q 'Finished'"
run_test "ARM32 release build" "cargo build --target armv7-cortex-kernel --release --bin kernel 2>&1 | grep -q 'Finished'"
run_test "RISC-V release build" "cargo build --target riscv64-cortex-kernel --release --bin kernel 2>&1 | grep -q 'Finished'"
echo ""

# Size tests
echo_color "33" "Size Constraint Tests"
echo_color "33" "----------------------"
./build/size_check.sh > /dev/null 2>&1 && run_test "All targets within 20KB limit" "true" || run_test "Size constraint check" "false"
echo ""

# QEMU availability tests (non-fatal)
echo_color "33" "QEMU Integration Tests"
echo_color "33" "----------------------"
if command -v qemu-system-x86_64 &> /dev/null; then
    echo_color "32" "✓ qemu-system-x86_64 available"
else
    echo_color "33" "⚠ qemu-system-x86_64 not found (install QEMU for integration tests)"
fi

if command -v qemu-system-aarch64 &> /dev/null; then
    echo_color "32" "✓ qemu-system-aarch64 available"
else
    echo_color "33" "⚠ qemu-system-aarch64 not found (install QEMU for integration tests)"
fi

if command -v qemu-system-arm &> /dev/null; then
    echo_color "32" "✓ qemu-system-arm available"
else
    echo_color "33" "⚠ qemu-system-arm not found (install QEMU for integration tests)"
fi

if command -v qemu-system-riscv64 &> /dev/null; then
    echo_color "32" "✓ qemu-system-riscv64 available"
else
    echo_color "33" "⚠ qemu-system-riscv64 not found (install QEMU for integration tests)"
fi
echo ""

# Summary
echo_color "35" "========================================="
echo_color "35" "Test Summary"
echo_color "35" "========================================="
echo "Total tests: $test_count"
echo_color "32" "Passed: $pass_count"
if [ $fail_count -gt 0 ]; then
    echo_color "31" "Failed: $fail_count"
    echo ""
    echo_color "31" "✗ Some tests failed"
    exit 1
else
    echo ""
    echo_color "32" "✓ All tests passed!"
    exit 0
fi
