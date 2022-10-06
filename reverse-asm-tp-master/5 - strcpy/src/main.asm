[bits 32]

[global asmEntry]

asmEntry:
    ;; FUNCTION PROLOGUE
    push ebp ;; We push the OLD stack base
    mov ebp, esp ;; We set the stack base to esp's new value
    push edi
    push ebx
    push ecx

    mov edi, [ebp+8]
    mov ebx, [ebp+12]
    mov ecx, 0

L1:
    cmp byte [ebx + ecx], 0
    jz END

    mov dl, byte [ebx + ecx]
    mov byte [edi + ecx], dl
    inc ecx
    jmp L1

END:
    pop edi
    pop ebx
    pop ecx
    leave
    ret
