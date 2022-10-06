[bits 32]

[global asmEntry]

asmEntry:
    ;; FUNCTION PROLOGUE
    push ebp ;; We push the OLD stack base
    mov ebp, esp ;; We set the stack base to esp's new value
    push ebx

    mov ecx, 0
    mov ebx, [ebp+8]

L1:
    cmp byte [ebx + ecx], 0
    jz END

    inc ecx
    jmp L1

END
    mov eax, ecx
    pop ebx
    leave
    ret
