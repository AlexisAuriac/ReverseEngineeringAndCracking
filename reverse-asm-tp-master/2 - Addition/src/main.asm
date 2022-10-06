[bits 32]

[global asmAdd]

asmAdd:
    ;; FUNCTION PROLOGUE
    push ebp ;; We push the OLD stack base
    mov ebp, esp ;; We set the stack base to esp's new value

    ; Write assembly code here!

    mov eax, [ebp+8]
    add eax, [ebp+12]

    ;; FUNCTION EPILOGUE
    leave
    ret
