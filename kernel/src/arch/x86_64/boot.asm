.section .multiboot
.align 8

# Multiboot2 header
.global multiboot_header
multiboot_header:
    .long 0x36d76289          # Multiboot2 magic
    .long 0x00000000          # Architecture (x86)
    .long multiboot_header_end - multiboot_header
    .long -(0x36d76289 + 0x00000000 + (multiboot_header_end - multiboot_header))

    # End tag
    .short 0                  # Type = 0 (end)
    .short 0                  # Flags = 0
    .long 8                   # Size = 8
multiboot_header_end:

.section .text.boot
.global _start

# Entry point that bootloader jumps to
_start:
    # Setup stack pointer
    movl $0x90000, %esp      # 64KB stack at 0x90000
    addl $0x10000, %esp
    
    # Load GDT
    lgdt gdt_descriptor
    
    # Enable protected mode
    movl %cr0, %eax
    orl $0x1, %eax           # Set PE bit
    movl %eax, %cr0
    
    # Far jump to enter 32-bit mode
    ljmp $0x08, $protected_mode

protected_mode:
    # Setup segment registers for 32-bit mode
    movw $0x10, %ax           # Data segment
    movw %ax, %ds
    movw %ax, %es
    movw %ax, %fs
    movw %ax, %gs
    movw %ax, %ss
    
    # Setup 32-bit stack
    movl $0x90000, %esp
    
    # Setup paging for long mode
    call setup_paging
    
    # Enable long mode
    call enable_long_mode
    
    # Far jump to enter 64-bit mode
    ljmp $0x08, $long_mode

long_mode:
    # Setup 64-bit segment registers
    movw $0x10, %ax           # Data segment
    movw %ax, %ds
    movw %ax, %es
    movw %ax, %fs
    movw %ax, %gs
    movw %ax, %ss
    
    # Setup 64-bit stack
    movq $0x90000, %rsp
    
    # Jump to Rust kernel
    call kernel_main
    
    # Halt if kernel returns
halt_loop:
    cli
    hlt
    jmp halt_loop

# Setup paging for long mode
setup_paging:
    # Load PML4 address into CR3
    movl $pml4_table, %eax
    movl %eax, %cr3
    
    # Enable PAE
    movl %cr4, %eax
    orl $(1 << 5), %eax       # Set PAE bit
    movl %eax, %cr4
    
    ret

# Enable long mode
enable_long_mode:
    # Enable long mode via EFER MSR
    movl $0xC0000080, %ecx    # EFER MSR number
    rdmsr                     # Read EFER
    orl $(1 << 8), %eax       # Set LME bit (Long Mode Enable)
    wrmsr                     # Write EFER
    
    # Enable paging to enter long mode
    movl %cr0, %eax
    orl $(1 << 31), %eax      # Set PG bit
    movl %eax, %cr0
    
    ret

# GDT descriptor
gdt_descriptor:
    .word gdt_end - gdt - 1    # Limit (size - 1)
    .quad gdt                  # Base address

# Global Descriptor Table
gdt:
    # Null descriptor (index 0)
    .quad 0x0000000000000000
    
    # Kernel code segment (index 1)
    .quad 0x0020980000000000   # 64-bit code segment, DPL=0
    
    # Kernel data segment (index 2) 
    .quad 0x0000920000000000   # 64-bit data segment, DPL=0
    
    # TSS descriptor (index 3) - placeholder
    .quad 0x0000000000000000
    
gdt_end:

# External function declaration
.global kernel_main
.type kernel_main, @function