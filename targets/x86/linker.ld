/**
 * Eciton - experimental exokernel.
 * Copyright (C) 2025 Alexander (@alkuzin).
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

/* boot - boot loader entry point. */
ENTRY(boot)
SECTIONS
{
    /* Conventional place for kernels to be loaded at by the bootloader. */
    base_address = 0xC0000000;
    . = 1M + base_address;

    kernel_virt_begin = .;
    kernel_phys_begin = kernel_virt_begin - base_address;

    .text ALIGN(4K) : AT(ADDR(.text) - base_address)
    {
        *(.multiboot)
        *(.text)        /* All code located in this section. */
    }

    /* Align read-only data (such as const variables) boundary. */
    .rodata ALIGN(4K) : AT(ADDR(.rodata) - base_address)
    {
        *(.rodata)
    }

    /* Read/write data (initialized). */
    .data ALIGN(4K) : AT(ADDR(.data) - base_address)
    {
        *(.data)
    }

    /* Global/static variables (unitialized). */
    .bss ALIGN(4K) : AT(ADDR(.bss) - base_address)
    {
        *(COMMON)
        *(.bss)
    }

    kernel_virt_end = .;
    kernel_phys_end = kernel_virt_end - base_address;
}