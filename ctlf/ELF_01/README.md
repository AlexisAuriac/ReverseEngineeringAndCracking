# ELF_01

The flag can be seen in raw binary file\
flag: Thiswasreallyeasy

Decompiled with Cutter:
```c
int32_t main (char ** argv, unsigned long argc) {
    rsi = argv;
    rdi = argc;
    if (edi != 2) {
        goto label_1;
    }
    rax = *((rsi + 8));
    ecx = 0x11;
    rdi = "Thiswasreallyeasy";
    rsi = rax;
    __asm ("repe cmpsb byte [rsi], byte ptr [rdi]");
    bl = (edi > 2) ? 1 : 0;
    ebx = (int32_t) bl;
    if (ebx == 0) {
        goto label_2;
    }
    rsi = rax;
    eax = 0;
    printf ("No, %s is not correct.\n");
    ebx = 1;
    do {
label_0:
        eax = 1;
        return rax;
label_1:
        rax = puts ("You should give one argument.");
        ebx = 0xffffffff;
    } while (1);
label_2:
    rsi = rax;
    eax = 0;
    printf ("Yes, %s is correct!\n");
    goto label_0;
}
```
