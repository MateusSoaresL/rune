; Do rune_print visible to linker.
global rune_print

; Do rune_println visible to linker.
global rune_println

; Program entry point.
global _start

; rune_main is defined in program.o
extern rune_main

; ASCII 10 = '\n'.
section .data
    newline db 10

; This section have an executable code.
section .text

; Function used by Rune to print a string.
; Entry:
;   RDI = string address.
;   RSI = string length.
rune_print:
    ; write() wait the length in RDX.
    mov rdx, rsi
    
    ; write() wait the string address in RSI.
    mov rsi, rdi

    ; RDI = 1 mean stdout.
    mov rdi, 1

    ; syscall 1 = write in Linux x86-64.
    mov rax, 1

    ; Call the kernel.
    syscall

    ; Come back to rune_main.
    ret

; Function used by Rune to print a string with line break.
; Entry:
;   RDI = string address.
;   RSI = string length.
rune_println:
    ; We need preserve registers, we will use after the first syscall.
    push rdi
    push rsi

    ; write() wait the length in RDX.
    mov rdx, rsi
    
    ; write() wait the string address in RSI.
    mov rsi, rdi

    ; RDI = 1 mean stdout.
    mov rdi, 1

    ; syscall 1 = write in Linux x86-64.
    mov rax, 1

    ; Call the kernel.
    syscall

    ; Recover the pile value.
    pop rsi
    pop rdi

    ; syscall 1 = write in Linux x86-64.
    mov rax, 1

    ; RDI = 1 mean stdout.
    mov rdi, 1

    ; '\n' address.
    lea rsi, [rel newline]

    ; Just 1 byte.
    mov rdx, 1

    ; Call the kernel.
    syscall

    ; Come back to rune_main.
    ret

; Executable entry point.
_start:
    ; Execute the compiled code by Rune.
    call rune_main

    ; rune_main returns the code in output in EAX.
    ; We put the code in EDI to exit().
    mov edi, eax

    ; syscall 60 = exit in Linux x86-64.
    mov rax, 60

    ; Finish the process.
    syscall