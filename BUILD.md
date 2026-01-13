# Cortex-μKernel Build System

A minimal, multi-architecture microkernel build system supporting x86_64, ARM64, ARM32, and RISC-V with QEMU integration and binary size optimization.

## Table of Contents

- [Prerequisites](#prerequisites)
- [Quick Start](#quick-start)
- [Build Targets](#build-targets)
- [Size Constraints](#size-constraints)
- [Platform-Specific Configuration](#platform-specific-configuration)
- [QEMU Integration](#qemu-integration)
- [Testing](#testing)
- [Debugging](#debugging)
- [Project Structure](#project-structure)

## Prerequisites

### Required Tools

- **Rust 1.70+** (nightly channel recommended)
  ```bash
  rustup default nightly
  rustup component add rust-src rust-analyzer
  ```

- **Cargo** (comes with Rust)

- **QEMU** (for emulation and testing)
  ```bash
  # Ubuntu/Debian
  sudo apt-get install qemu-system-x86 qemu-system-arm qemu-system-misc

  # macOS
  brew install qemu

  # Fedora
  sudo dnf install qemu-system-x86 qemu-system-arm qemu-system-riscv
  ```

- **LLVM** (included with Rust toolchain)

### Optional Tools

- **GDB** for debugging:
  ```bash
  sudo apt-get install gdb-multiarch
  ```

- **size** utility (usually included with binutils):
  ```bash
  sudo apt-get install binutils
  ```

## Quick Start

```bash
# Clone the repository
git clone <repository-url>
cd cortex-mu-kernel

# Build all targets
./build/build.sh all

# Run size check
./build/size_check.sh

# Run tests
./build/test.sh

# Boot x86_64 kernel in QEMU
./build/qemu_x86_64.sh
```

## Build Targets

The build system supports four target architectures:

| Target | Description | Status |
|--------|-------------|--------|
| `x86_64` | x86_64 (Intel/AMD 64-bit) | ✓ Active |
| `arm64` | AArch64 (ARM 64-bit) | ✓ Active |
| `arm32` | ARMv7 (ARM 32-bit) | ✓ Active |
| `riscv` | RISC-V 64-bit | ✓ Active |

### Building Individual Targets

```bash
# Build x86_64
./build/build.sh x86_64

# Build ARM64
./build/build.sh arm64

# Build ARM32
./build/build.sh arm32

# Build RISC-V
./build/build.sh riscv

# Build all targets
./build/build.sh all
```

### Clean Build Artifacts

```bash
./build/build.sh clean
```

## Size Constraints

All kernel binaries are optimized for minimal size with a **20KB limit** per target.

### Size Optimization Flags

The `Cargo.toml` workspace uses these release profile settings:

```toml
[profile.release]
opt-level = "z"      # Minimize size
lto = true           # Link-time optimization
codegen-units = 1    # Single codegen unit for better LTO
strip = true         # Strip debug symbols
panic = "abort"      # Abort on panic (no unwinding)
```

### Checking Binary Sizes

```bash
./build/size_check.sh
```

Example output:
```
==========================================
Cortex-μKernel Binary Size Report
==========================================

x86_64-cortex-kernel:
  Size: 12 KB (12288 bytes)
  Limit: 20 KB (20480 bytes)
  ✓ WITHIN LIMIT

aarch64-cortex-kernel:
  Size: 14 KB (14336 bytes)
  Limit: 20 KB (20480 bytes)
  ✓ WITHIN LIMIT

...
```

## Platform-Specific Configuration

### Target Specifications

Custom target JSON files are located in `.cargo/`:

- `x86_64-cortex-kernel.json` - x86_64 with no FPU
- `aarch64-cortex-kernel.json` - ARM64 with soft-float, no NEON
- `armv7-cortex-kernel.json` - ARM32 Thumb mode
- `riscv64-cortex-kernel.json` - RISC-V 64 generic

### Linker Scripts

Platform-specific linker scripts in `build/`:

| Script | Platform | Memory Origin |
|--------|----------|---------------|
| `x86_64.ld` | x86_64 | 0x100000 (1MB) |
| `arm64.ld` | ARM64 | 0x40000000 (1GB) |
| `arm32.ld` | ARM32 | 0x00000000 (0MB) |
| `riscv64.ld` | RISC-V 64 | 0x80200000 (2GB) |

### Cargo Config

`.cargo/config.toml` defines per-target Rust flags:

```toml
[target.x86_64-cortex-kernel]
rustflags = [
    "-C", "link-arg=-nostartfiles",
    "-C", "link-arg=-Tbuild/x86_64.ld",
]
```

## QEMU Integration

### Starting QEMU

Each target has a dedicated QEMU boot script:

```bash
# x86_64
./build/qemu_x86_64.sh

# ARM64
./build/qemu_arm64.sh

# ARM32
./build/qemu_arm32.sh

# RISC-V 64
./build/qemu_riscv64.sh
```

### QEMU Machine Configurations

| Platform | Machine | CPU | Memory |
|----------|---------|-----|--------|
| x86_64 | Default | x86_64 | 256MB |
| ARM64 | virt | cortex-a57 | 256MB |
| ARM32 | mps2-an505 | cortex-m4 | 256MB |
| RISC-V 64 | virt | generic-rv64 | 256MB |

### GDB Debugging

All QEMU scripts include `-s -S` flags for GDB debugging:

```bash
# Terminal 1: Start QEMU
./build/qemu_x86_64.sh

# Terminal 2: Connect GDB
gdb-multiarch target/x86_64-cortex-kernel/release/kernel
(gdb) target remote :1234
(gdb) break _start
(gdb) continue
```

## Testing

### Running All Tests

```bash
./build/test.sh
```

The test suite includes:

1. **Unit Tests** - Library tests for verifier, kernel, slots
2. **Build Tests** - Verifies all targets compile
3. **Size Tests** - Checks binary size constraints
4. **QEMU Availability** - Validates QEMU installation

### Running Specific Tests

```bash
# Unit tests only
cargo test --lib --release

# Verifier tests
cargo test --lib verifier --release

# Kernel library tests
cargo test --lib kernel --release
```

## Debugging

### Build with Debug Symbols

For debugging, build without strip:

```bash
cargo build --target x86_64-cortex-kernel --release
```

The release profile is already optimized; for better debugging info, modify `[profile.release]` in `Cargo.toml`:

```toml
[profile.release]
opt-level = "z"
lto = true
codegen-units = 1
# strip = true  # Comment out for debugging symbols
```

### GDB Commands

Common GDB commands for kernel debugging:

```gdb
# Connect to QEMU
target remote :1234

# Set breakpoints
break _start
break kernel_main

# Continue execution
continue

# Step through code
stepi
nexti

# Examine registers
info registers

# Examine memory
x/10x 0x100000

# Disassemble
disassemble _start
```

## Project Structure

```
cortex-mu-kernel/
├── .cargo/
│   ├── config.toml              # Cargo configuration
│   ├── x86_64-cortex-kernel.json
│   ├── aarch64-cortex-kernel.json
│   ├── armv7-cortex-kernel.json
│   └── riscv64-cortex-kernel.json
├── build/
│   ├── build.sh                 # Main build orchestrator
│   ├── size_check.sh            # Binary size reporter
│   ├── test.sh                  # Test suite
│   ├── qemu_x86_64.sh           # x86_64 QEMU runner
│   ├── qemu_arm64.sh            # ARM64 QEMU runner
│   ├── qemu_arm32.sh            # ARM32 QEMU runner
│   ├── qemu_riscv64.sh          # RISC-V QEMU runner
│   ├── x86_64.ld                # x86_64 linker script
│   ├── arm64.ld                 # ARM64 linker script
│   ├── arm32.ld                 # ARM32 linker script
│   └── riscv64.ld               # RISC-V linker script
├── kernel/                      # Main kernel library
├── verifier/                    # Verification library
├── slots/                       # Slot management library
├── Cargo.toml                   # Workspace configuration
├── rust-toolchain.toml          # Toolchain specification
├── BUILD.md                     # This file
└── LICENSE
```

## Additional Examples

### Building for a specific target triple

```bash
# Explicit target specification
cargo build --target x86_64-cortex-kernel --release --bin kernel
```

### Checking build output

```bash
# View binary size with size command
size -A target/x86_64-cortex-kernel/release/kernel

# View binary details
file target/x86_64-cortex-kernel/release/kernel

# List symbols
nm target/x86_64-cortex-kernel/release/kernel
```

### Continuous Integration

The build system is CI-ready. Example GitHub Actions workflow:

```yaml
name: Build and Test

on: [push, pull_request]

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          profile: minimal
          toolchain: nightly
          components: rust-src
      - name: Install QEMU
        run: sudo apt-get install qemu-system-x86 qemu-system-arm qemu-system-misc
      - name: Build all targets
        run: ./build/build.sh all
      - name: Run tests
        run: ./build/test.sh
      - name: Check sizes
        run: ./build/size_check.sh
```

## Troubleshooting

### "Linker not found" error

Ensure the LLVM toolchain is installed:
```bash
rustup component add llvm-tools-preview
```

### "Target not found" error

The custom targets must be in `.cargo/`. Verify the JSON files are present.

### QEMU won't start

1. Ensure QEMU is installed and accessible
2. Check that the kernel binary exists
3. Try running QEMU with `-d int,cpu_reset` for debugging

### Binary size too large

1. Review the release profile settings in `Cargo.toml`
2. Check for unnecessary dependencies
3. Use `cargo bloat` to identify large sections:
   ```bash
   cargo install cargo-bloat
   cargo bloat --release --crates
   ```

## License

Apache License 2.0 - see LICENSE file for details.
