ASM    		 = as
ASM_FLAGS    = --32
LINKER 		 = ld
LINKER_FLAGS = -z noexecstack -melf_i386

KERNEL_PATH = kernel
BUILD_PATH  = build
ISO_PATH    = $(BUILD_PATH)/iso
ARCH_PATH   = $(KERNEL_PATH)/arch
TARGET_PATH = $(ARCH_PATH)/i686/boot

NAME 		= eciton
ISO_NAME    = $(BUILD_PATH)/$(NAME).iso
KERNEL_ELF  = $(ISO_PATH)/boot/$(NAME).elf

KERNEL_STATIC_LIB = $(KERNEL_PATH)/target/i686-unknown-none/release/libeciton.a

OBJS = $(TARGET_PATH)/boot.o \
	   $(KERNEL_STATIC_LIB)

$(NAME):
	cargo build --manifest-path $(KERNEL_PATH)/Cargo.toml --release
	$(ASM) $(ASM_FLAGS) -o $(TARGET_PATH)/boot.o $(TARGET_PATH)/boot.asm
	$(LINKER) $(LINKER_FLAGS) -o $(KERNEL_ELF) -T $(TARGET_PATH)/linker.ld $(OBJS)

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
	cp $(TARGET_PATH)/../grub.cfg $(ISO_PATH)/boot/grub/grub.cfg
	grub-mkrescue -o $(ISO_NAME) $(ISO_PATH)

init:
	qemu-system-i386 -m 256 -cdrom $(ISO_NAME)

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