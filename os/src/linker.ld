OUTPUT_ARCH(riscv)
ENTRY(_start)
BASE_ADDRESS = 0x80200000;

SECTIONS
{
    . = BASE_ADDRESS;
    skernel = .;

    stext = .;
    .text : {
        *(.text.entry)
        *(.text .text.*)
    }

    . = ALIGN(4K);
    etext = .;
    srodata = .;
    .rodata : {
        *(.rodata .rodata.*)
    }

    . = ALIGN(4K);
    erodata = .;
    sdata = .;
    .data : {
        *(.data .data.*)
    }

    . = ALIGN(4K);
    edata = .;
    .bss : {
        *(.bss.stack)
        sbss = .;
        *(.bss .bss.*)
    }

    . = ALIGN(4K);
    ebss = .;
    ekernel = .;


 /* 明确布局调试信息节 */
    .debug_info : {
        *(.debug_info)
    }
    .debug_abbrev : {
        *(.debug_abbrev)
    }
    .debug_loc : {
        *(.debug_loc)
    }
    /* 可以根据实际情况添加更多调试信息节 */


    /DISCARD/ : {
        *(.eh_frame)
    }
}
