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

ASM    		 = as
ASM_FLAGS    = --32
LINKER 		 = ld
LINKER_FLAGS = -z noexecstack -melf_i386

SELECTED_TARGET = i686

KERNEL_PATH  	 = kernel
BUILD_PATH   	 = build
ISO_PATH     	 = $(BUILD_PATH)/iso
ARCH_PATH    	 = $(KERNEL_PATH)/src/eciton/arch
ASM_PATH     	 = $(ARCH_PATH)/$(SELECTED_TARGET)/asm
TARGETS_PATH 	 = targets/$(SELECTED_TARGET)
GRUB_CONFIG_PATH = targets

NAME 	   		  = eciton
ISO_NAME   		  = $(BUILD_PATH)/$(NAME).iso
KERNEL_ELF 		  = $(ISO_PATH)/boot/$(NAME).elf
KERNEL_STATIC_LIB = $(KERNEL_PATH)/target/$(SELECTED_TARGET)-unknown-none/debug/libeciton.a

ASM_SRC  = $(ASM_PATH)/boot \
		   $(ASM_PATH)/gdt_flush \
		   $(ASM_PATH)/idt_flush
ASM_SRCS = $(addsuffix .asm, $(ASM_SRC))
ASM_OBJS = $(addsuffix .o,   $(ASM_SRC))

OBJS = $(ASM_OBJS) $(KERNEL_STATIC_LIB)

$(ASM_PATH)/%.o: $(ASM_PATH)/%.asm
	$(ASM) $(ASM_FLAGS) -c $< -o $@

$(NAME): $(OBJS)
	cargo build --manifest-path $(KERNEL_PATH)/Cargo.toml
	$(LINKER) $(LINKER_FLAGS) -o $(KERNEL_ELF) -T $(TARGETS_PATH)/linker.ld $(OBJS)

$(ISO_PATH):
	mkdir -p $(ISO_PATH)/boot/grub/

$(BUILD_PATH):
	mkdir -p $(BUILD_PATH)

all: check $(BUILD_PATH) $(ISO_PATH) $(NAME)

clean:
	rm -f $(OBJS) $(KERNEL_ELF)

fclean: clean
	rm -f $(ISO_NAME)
	rm -rf $(BUILD_PATH)/
	cargo clean --manifest-path $(KERNEL_PATH)/Cargo.toml

re: fclean all

build-iso: all
	cp $(GRUB_CONFIG_PATH)/grub.cfg $(ISO_PATH)/boot/grub/grub.cfg
	grub-mkrescue -o $(ISO_NAME) $(ISO_PATH)

init:
	qemu-system-i386 -m 256 -cdrom $(ISO_NAME) -serial stdio

run: build-iso init

check-clippy:
	cargo clippy --manifest-path $(KERNEL_PATH)/Cargo.toml -- -D warnings -W clippy::all

check: check-clippy
	cargo check --manifest-path $(KERNEL_PATH)/Cargo.toml

build-doc:
	cargo doc --document-private-items --manifest-path $(KERNEL_PATH)/Cargo.toml

doc:
	cargo doc --document-private-items --open --manifest-path $(KERNEL_PATH)/Cargo.toml

debug:
	qemu-system-i386 -s -S -m 256 -cdrom $(ISO_NAME) & gdb $(KERNEL_ELF) -ex "target remote localhost:1234" -tui