[bits 32]

[global callfcn]

callfcn:
    ;; FUNCTION PROLOGUE
    push ebp
    mov ebp, esp

    mov ecx, [ebp+12] ; number arguments

L1:
    cmp ecx, 0
    jz END

    push dword [ebp+12+ecx*4]
    sub ecx, 1

    jmp L1

END:
    call [ebp+8]

    mov esp, ebp
    pop ebp
    ret
