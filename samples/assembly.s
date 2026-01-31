; Assembly x86-64 Example
; Hello World program for Linux

section .data
    msg     db  'Hello, World!', 0xa     ; string with newline
    len     equ $ - msg                   ; length of string

section .text
    global _start

_start:
    ; write(stdout, msg, len)
    mov     rax, 1          ; syscall: write
    mov     rdi, 1          ; file descriptor: stdout
    mov     rsi, msg        ; buffer
    mov     rdx, len        ; count
    syscall

    ; exit(0)
    mov     rax, 60         ; syscall: exit
    xor     rdi, rdi        ; status: 0
    syscall

; Function example
multiply:
    push    rbp
    mov     rbp, rsp
    mov     eax, edi        ; first argument
    imul    eax, esi        ; multiply by second argument
    pop     rbp
    ret

; Data structures
section .bss
    buffer  resb 64         ; reserve 64 bytes
    count   resd 1          ; reserve 1 dword (4 bytes)

section .rodata
    fmt     db  '%d', 0xa, 0
    pi      dq  3.14159265358979
