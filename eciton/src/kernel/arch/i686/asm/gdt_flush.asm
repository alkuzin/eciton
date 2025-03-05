# Eciton - experimental exokernel.
# Copyright (C) 2025 Alexander (@alkuzin).
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with this program.  If not, see <https://www.gnu.org/licenses/>.

.global gdt_flush

gdt_flush:
    mov  4(%esp), %eax  # Get function argument from stack.
    lgdt (%eax)         # Load Global Descriptor Table.

                        # Reload data segment registers.

    mov $0x10, %eax     # Kernel data segment selector.
    mov %ax, %ds        # Allow the CPU to access to kernel data segment.
    mov %ax, %es        # Allow the CPU to access to kernel extra segment.
    mov %ax, %fs        # Allow the CPU to access to kernel additional segment.
    mov %ax, %gs        # Allow the CPU to access to kernel global segment.
    mov $0x18, %ax      # Offset in the GDT to kernel stack.
    mov %ax, %ss        # Allow the CPU to access to kernel stack segment.

    jmp $0x08, $flush   # Return to kernel code segment.

flush:
    ret                 # Return back to Rust code.