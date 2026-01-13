# Cortex-μKernel

A minimal, multi-architecture microkernel supporting x86_64, ARM64, ARM32, and RISC-V platforms.

## Features

- **Multi-architecture support**: x86_64, ARM64, ARM32, RISC-V 64
- **Size-optimized**: Target <20KB per platform with LTO and aggressive optimization
- **No external dependencies**: Pure Rust + assembly
- **Modular design**: Separate kernel, verifier, and slot management crates
- **QEMU integration**: Ready-to-run emulation for all platforms
- **Comprehensive build system**: Automated building, testing, and size checking

## Quick Start

```bash
# Install Rust nightly
rustup default nightly
rustup component add rust-src rust-analyzer

# Build all targets
./build/build.sh all

# Check binary sizes
./build/size_check.sh

# Run tests
./build/test.sh

# Boot in QEMU (x86_64)
./build/qemu_x86_64.sh
```

## Documentation

- **[BUILD.md](BUILD.md)** - Complete build system documentation
- **[LICENSE](LICENSE)** - Apache License 2.0

## Project Structure

```
cortex-mu-kernel/
├── kernel/          # Main kernel binary
├── verifier/        # Kernel verification library
├── slots/           # Slot management library
├── build/           # Build scripts and linker scripts
├── .cargo/          # Target specifications and config
└── .github/         # CI/CD workflows
```

## Supported Platforms

| Platform | QEMU Machine | Status |
|----------|--------------|--------|
| x86_64   | Default      | ✓ Active |
| ARM64    | virt         | ✓ Active |
| ARM32    | mps2-an505   | ✓ Active |
| RISC-V 64| virt         | ✓ Active |

## Size Constraints

All kernel binaries are optimized to be under **20KB** per platform.

## Development

This project uses the Rust nightly toolchain for:
- Custom target specifications
- Inline assembly support
- No-std kernel development

## License

Apache License 2.0
