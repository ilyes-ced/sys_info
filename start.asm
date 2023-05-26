# comment

.global _start
.section .text

_start:

    mov r15, #8

loop:
    cmp r15, #0
    beq exit

    bl hello_world

    sub r15, r15, #1
    b loop


hello_world:
    mov r7, #0x4
    mov r0, #1
    ldr r1, =message
    mov r2, #13
    swi 0
    ret


exit:
    mov r7, #0x1
    mov r0, #65
    swi 0

.section .data
    message:
    .ascii "hello world\n"