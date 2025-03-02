# Eciton exokernel documentation

## Contents

1. [Introduction](#1-introduction)
2. [Tools installation](#2-tools-installation)
3. [Installation](#3-installation)

## 1. Introduction
Eciton - experimental x86-32 exokernel written in Rust

## 2. Tools installation

List of tools:
 - rustup 1.27.1 (2024-12-25)
 - rustc 1.86.0-nightly (f7cc13af8 2025-01-25)
 - GNU Make (4.4.1)
 - GNU Binutils for Debian (2.44)
 - QEMU emulator (9.2.0)
 - GNU GRUB (2.12-5)

For Debian/Ubuntu:
```console
sudo apt install make qemu-system-x86 binutils rustup
sudo apt-get install grub-pc-bin mtools
```

Then setup rust toolchain (cargo, clippy)

## 3. Installation
First clone this repository:
```console
https://github.com/alkuzin/eciton.git
```

To build kernel as `.iso` and run on QEMU use:

```console
make run
```

To return everything to original state:
```console
make fclean
```

To build documentation run:
```console
make build-doc
```

To build and open documentation in browser run:
```console
make doc
```