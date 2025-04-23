.text
.align 2

.globl add
add:
    addi sp, sp, -16
    sd ra, 8(sp)
    sd fp, 0(sp)
    addi fp, sp, 16
.Lreturn:
    ld ra, 8(sp)
    ld fp, 0(sp)
    addi sp, sp, 16
    ret

.globl subtract
subtract:
    addi sp, sp, -16
    sd ra, 8(sp)
    sd fp, 0(sp)
    addi fp, sp, 16
.Lreturn:
    ld ra, 8(sp)
    ld fp, 0(sp)
    addi sp, sp, 16
    ret

.globl multiply
multiply:
    addi sp, sp, -16
    sd ra, 8(sp)
    sd fp, 0(sp)
    addi fp, sp, 16
.Lreturn:
    ld ra, 8(sp)
    ld fp, 0(sp)
    addi sp, sp, 16
    ret

.globl divide
divide:
    addi sp, sp, -16
    sd ra, 8(sp)
    sd fp, 0(sp)
    addi fp, sp, 16
.Lreturn:
    ld ra, 8(sp)
    ld fp, 0(sp)
    addi sp, sp, 16
    ret

