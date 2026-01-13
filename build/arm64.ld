ENTRY(_start)

MEMORY
{
  KERNEL : ORIGIN = 0x40000000, LENGTH = 64K
}

SECTIONS
{
  . = ORIGIN(KERNEL);

  .boot : {
    KEEP(*(.boot.text))
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
    *(.ARM.exidx*)
    *(.ARM.extab*)
    *(.eh_frame*)
    *(.eh_frame_hdr*)
  }
}
