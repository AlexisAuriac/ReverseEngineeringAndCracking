[bits 32]

[global asmEntry]

asmEntry:
    ;; FUNCTION PROLOGUE
    push ebp ;; We push the OLD stack base
    mov ebp, esp ;; We set the stack base to esp's new value
    push edi
    push ecx

    mov edi, [ebp+8]
    mov eax, 0
    mov ecx, 0FFFFFFFFh

    repne scasb
    not ecx
    dec ecx
    mov eax, ecx

    pop edi
    pop ecx
    leave
    ret
