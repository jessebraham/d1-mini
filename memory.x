/* Linker script for the ESP8266 */

MEMORY
{
    /* All .data/.bss/heap are in this segment.
       Reserve 1KB for old boot or ROM boot. */
    dram_seg :      org = 0x3FFE8000, len = 0x18000 - 0x400

    /* Functions which are critical should be put in this segment. */
    iram_seg :      org = 0x40100000, len = 0xFC00
}
