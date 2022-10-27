# ELF_02

See ELF_02.c for a approximation of the original code.

Cutter decompiled:
```c
/* jsdec pseudo code output */
/* /home/alexis/Documents/tech5_2/reverse_engineering_and_cracking/ctlf/ELF_02/ELF_02 @ 0x1149 */
#include <stdint.h>
 
int32_t main (char ** argv, unsigned long argc) {
    rsi = argv;
    rdi = argc;
    if (edi != 2) {
        goto label_1;
    }
    rsi = *((rsi + 8)); // argv[1]
    rcx = 0xffffffffffffffff; // rcx = 1
    eax = 0;
    rdi = rsi;
    __asm ("repne scasb al, byte [rdi]");
    edx = 0x70; // 'p'
    edi = 0; // i = 0
    r8 = "password424242";
    if (rcx != 0xfffffffffffffff0) {
        goto label_2;
    }
    do {
        eax = *((rsi + rdi)); // argv[1][i]
        if (al == 0) {
            goto label_3;
        }
        edx = (int32_t) dl;
        edx--;
        eax = (int32_t) al;
        if (edx != eax) {
            goto label_4;
        }
        rdi++;
        edx = *((r8 + rdi));
    } while (dl != 0);
label_3:
    eax = 0;
    printf ("Yes, %s is correct!\n");
    eax = 0;
    goto label_0;
label_1:
    puts ("You should give one argument.");
    eax = 0xffffffff;
    do {
label_0:
        return eax;
label_2:
        printf ("No, %s is not correct.\n");
        eax = 0;
    } while (1);
label_4:
    eax = 0;
    printf ("No, %s is not correct.\n");
    eax = 1;
    goto label_0;
}
```

The flag is the string **password424242** but every character was decremented, 'p' becomes 'o', 's' becomes 'r', etc...

flag: ```o`rrvnqc313131```