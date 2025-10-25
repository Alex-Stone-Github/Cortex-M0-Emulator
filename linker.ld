MEMORY {
	FLASH (rwx) : ORIGIN = 0, LENGTH = 500
}

SECTIONS
{
    .text :
    {
        *(.text)
		*(.rodata)
		*(.rodata.*)
    } > FLASH
	.data : 
	{
		*(.data)
		*(.bss)
	} > FLASH
}
