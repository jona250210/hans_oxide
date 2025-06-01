/* Fully featured memory.x file */
MEMORY
{
  SDRAM : ORIGIN = 0x00000000, LENGTH = 16M
}

REGION_ALIAS("REGION_TEXT", SDRAM);
REGION_ALIAS("REGION_RODATA", SDRAM);
REGION_ALIAS("REGION_DATA", SDRAM);
REGION_ALIAS("REGION_BSS", SDRAM);
REGION_ALIAS("REGION_HEAP", SDRAM);
REGION_ALIAS("REGION_STACK", SDRAM);

_stext = ORIGIN(REGION_TEXT) + 0x400000;        /* Skip first 4M of text region */
_heap_size = 1K;                                /* Set heap size to 1KB */
_max_hart_id = 1;                               /* Two harts present */
_hart_stack_size = 1K;                          /* Set stack size per hart to 1KB */
_stack_start = ORIGIN(SDRAM) + LENGTH(SDRAM);
