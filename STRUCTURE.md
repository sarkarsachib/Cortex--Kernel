# Cortex-μKernel Build System - File Structure

This document provides a comprehensive overview of the project structure created for the multi-architecture microkernel build system.

## Root Directory

```
cortex-mu-kernel/
├── .cargo/                          # Cargo configuration directory
│   ├── config.toml                   # Cargo configuration with per-target flags
│   ├── x86_64-cortex-kernel.json    # x86_64 target specification
│   ├── aarch64-cortex-kernel.json   # ARM64 target specification
│   ├── armv7-cortex-kernel.json     # ARM32 target specification
│   └── riscv64-cortex-kernel.json   # RISC-V 64 target specification
│
├── .github/workflows/                # CI/CD workflows
│   └── build.yml                    # GitHub Actions build workflow
│
├── build/                           # Build system scripts
│   ├── build.sh                      # Main build orchestrator
│   ├── size_check.sh                 # Binary size reporter
│   ├── test.sh                       # Test suite runner
│   ├── qemu_x86_64.sh               # x86_64 QEMU boot script
│   ├── qemu_arm64.sh                # ARM64 QEMU boot script
│   ├── qemu_arm32.sh                # ARM32 QEMU boot script
│   ├── qemu_riscv64.sh              # RISC-V 64 QEMU boot script
│   ├── x86_64.ld                    # x86_64 linker script
│   ├── arm64.ld                     # ARM64 linker script
│   ├── arm32.ld                     # ARM32 linker script
│   └── riscv64.ld                  # RISC-V 64 linker script
│
├── kernel/                          # Main kernel crate
│   ├── Cargo.toml                   # Kernel crate configuration
│   └── src/
│       ├── main.rs                  # Kernel entry point (bin)
│       └── lib.rs                   # Kernel library
│
├── verifier/                        # Verification library crate
│   ├── Cargo.toml                   # Verifier crate configuration
│   └── src/
│       └── lib.rs                   # Verification library with tests
│
├── slots/                           # Slot management library crate
│   ├── Cargo.toml                   # Slots crate configuration
│   └── src/
│       └── lib.rs                   # Slot management library with tests
│
├── Cargo.toml                       # Workspace configuration
├── rust-toolchain.toml              # Rust toolchain specification
├── BUILD.md                         # Comprehensive build documentation
├── README.md                        # Project overview
├── .gitignore                       # Git ignore rules
└── LICENSE                          # Apache License 2.0
```

## File Descriptions

### Workspace Configuration

**Cargo.toml**
- Defines workspace with 3 members: kernel, verifier, slots
- Shared workspace dependencies
- Release profile: opt-level="z", lto=true, codegen-units=1, strip=true
- No external dependencies

**rust-toolchain.toml**
- Specifies nightly Rust channel
- Includes rust-src and rust-analyzer components

**.gitignore**
- Rust target directory
- Editor configuration files
- Build artifacts
- QEMU disk images

### Target Specifications (.cargo/*.json)

**x86_64-cortex-kernel.json**
- llvm-target: x86_64-unknown-none
- Features: -mmx, -sse, +soft-float
- Panic strategy: abort
- Linker: rust-lld

**aarch64-cortex-kernel.json**
- llvm-target: aarch64-unknown-none
- Features: -neon, -fp-armv8, +soft-float, +strict-align
- Panic strategy: abort
- Linker: rust-lld

**armv7-cortex-kernel.json**
- llvm-target: armv7em-none-eabihf
- Features: +vfp4, +d16, +thumb-mode, -neon
- CPU: cortex-m4
- Panic strategy: abort

**riscv64-cortex-kernel.json**
- llvm-target: riscv64-unknown-none
- Features: +m, +a, +f, +d, +c
- CPU: generic-rv64
- Panic strategy: abort

### Cargo Configuration (.cargo/config.toml)

- Default target: x86_64-cortex-kernel
- Per-target rustflags:
  - nostartfiles flag
  - Linker script reference
  - Architecture-specific features

### Linker Scripts (build/*.ld)

**x86_64.ld**
- Entry point: _start
- Memory region: 0x100000 (1MB), length 64K
- Sections: .boot, .text, .rodata, .data, .bss
- Discards: note, comment, eh_frame sections

**arm64.ld**
- Entry point: _start
- Memory region: 0x40000000 (1GB), length 64K
- Sections: .boot, .text, .rodata, .data, .bss
- Discards ARM-specific sections

**arm32.ld**
- Entry point: _start
- Memory region: 0x00000000, length 64K
- Sections: .boot with vectors, .text, .rodata, .data, .bss
- Thumb-2 code support

**riscv64.ld**
- Entry point: _start
- Memory region: 0x80200000 (2GB), length 64K
- Sections: .boot with vectors, .text, .rodata, .data, .bss

### Build Scripts

**build/build.sh**
- Main orchestrator for all targets
- Functions: build_x86_64(), build_arm64(), build_arm32(), build_riscv(), build_all()
- Color-coded output
- Error handling with set -e
- Options: x86_64, arm64, arm32, riscv, all, clean, help

**build/size_check.sh**
- Reports binary sizes for all targets
- 20KB size limit enforcement
- Color-coded pass/fail status
- Detailed summary report

**build/test.sh**
- Comprehensive test suite
- Unit tests for all crates
- Build tests for all targets
- Size constraint tests
- QEMU availability checks
- Detailed summary with pass/fail counts

### QEMU Boot Scripts

**build/qemu_x86_64.sh**
- Machine: default
- CPU: x86_64
- Memory: 256MB
- Flags: -serial stdio, -nographic, -m 256M, -s -S

**build/qemu_arm64.sh**
- Machine: virt
- CPU: cortex-a57
- Memory: 256MB
- Flags: -serial stdio, -nographic, -m 256M, -s -S

**build/qemu_arm32.sh**
- Machine: mps2-an505
- CPU: cortex-m4
- Memory: 256MB
- Flags: -serial stdio, -nographic, -m 256M, -s -S

**build/qemu_riscv64.sh**
- Machine: virt
- CPU: generic-rv64
- Memory: 256MB
- Flags: -bios none, -serial stdio, -nographic, -m 256M, -s -S

### Crates

**kernel/**
- main.rs: Kernel entry point with platform-specific boot sequences
- lib.rs: Kernel library functions
- Dependencies: verifier, slots

**verifier/**
- lib.rs: Verification library with tests
- Tests: test_verifier_creation, test_verifier_default, test_verify_kernel

**slots/**
- lib.rs: Slot management library with tests
- Tests: test_slot_creation, test_slot_activation, test_slot_manager_creation, test_slot_allocate, test_slot_deallocate, test_max_slots

### Documentation

**BUILD.md**
- Prerequisites and installation
- Quick start guide
- Build target descriptions
- Size constraints documentation
- Platform-specific configuration
- QEMU integration
- Testing procedures
- Debugging guide
- Troubleshooting section

**README.md**
- Project overview
- Features summary
- Quick start
- Platform support table
- Size constraints
- Development notes

### CI/CD

**.github/workflows/build.yml**
- Triggers: push, pull_request
- Jobs: build (matrix), test, build-all
- Caches: cargo registry, index, build
- Uploads build artifacts
- Reports binary sizes

## Usage Examples

### Building Individual Targets

```bash
./build/build.sh x86_64
./build/build.sh arm64
./build/build.sh arm32
./build/build.sh riscv
```

### Building All Targets

```bash
./build/build.sh all
```

### Running Tests

```bash
./build/test.sh
```

### Checking Binary Sizes

```bash
./build/size_check.sh
```

### Booting in QEMU

```bash
./build/qemu_x86_64.sh
./build/qemu_arm64.sh
./build/qemu_arm32.sh
./build/qemu_riscv64.sh
```

## Acceptance Criteria Checklist

- [x] All 4 target configurations complete (x86_64, arm64, arm32, riscv)
- [x] Linker scripts work for each platform
- [x] Build script supports individual targets
- [x] Build script supports building all targets
- [x] QEMU boot scripts for all platforms
- [x] Binary size reporting works with 20KB limit
- [x] Test runner executes all tests
- [x] No external dependencies (pure Rust + ASM)
- [x] BUILD.md complete with examples
- [x] Rustup/cargo toolchain configured
- [x] CI/CD workflow for GitHub Actions
- [x] Clean project structure
- [x] Comprehensive documentation
