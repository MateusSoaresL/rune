; Do rune_print visible to linker.
global rune_print

; Program entry point.
global _start

; rune_main is defined in program.o
extern rune_main

; This section have an executable code.
section .text

; Function used by RUne to print a string.
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