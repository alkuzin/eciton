# Eciton exokernel changes log

## release v0.0.1

|Language|files|blank|comment|code|
|--------|-----|-----|-------|----|
|Rust    |    8|  100|    288| 691|
|Assembly|    1|   23|     27|  67|
|SUM:    |    9|  123|    315| 758|

| Total lines | Size (with debug symbols) | Pure size             |
|-------------|---------------------------|-----------------------|
| 1196        | 552328 bytes (540 KB)     | 458200 bytes (448 KB) |

* feat: added basic `panic` handler
* feat: added kernel tty & `printk!` `putk!` macros
* feat: added `fill_screen` method
* feat: added debug function
* feat: added additional checks and documentation creating commands
* feat: added `vesa` driver
* feat: added `graphics module` and `font`
* feat: added `multiboot` info structs
* Initial commit