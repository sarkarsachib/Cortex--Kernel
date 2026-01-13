ENTRY(_start)

MEMORY
{
  KERNEL : ORIGIN = 0x0000000000100000, LENGTH = 64K
}

SECTIONS
{
  . = ORIGIN(KERNEL);

  .boot : {
    *(.multiboot)
    . = ALIGN(4K);
  } > KERNEL

  .text : {
    *(.text .text.*)
    . = ALIGN(4K);
  } > KERNEL

  .rodata : {
    *(.rodata .rodata.*)
    . = ALIGN(4K);
  } > KERNEL

  .data : {
    *(.data .data.*)
    . = ALIGN(4K);
  } > KERNEL

  .bss : {
    __bss_start = .;
    *(.bss .bss.*)
    *(COMMON)
    __bss_end = .;
    . = ALIGN(4K);
  } > KERNEL

  _kernel_end = .;

  /DISCARD/ : {
    *(.note*)
    *(.comment*)
    *(.eh_frame*)
    *(.eh_frame_hdr*)
  }
}
