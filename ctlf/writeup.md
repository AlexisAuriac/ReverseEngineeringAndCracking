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

# ELF_03

See ELF_03.c for a approximation of the original code.

secret: MasKerade133742A
mask: 0x0503020302

password = secret + mask

See solve.c for an implementation of the solution.

# ELF_04

## What it does

Checks if the password is 16 characters long.

Checks the sum of the characters is equal to 1664.

## Solution

1664 / 16 = 104 = 'h'

One possible password is "hhhhhhhhhhhhhhhh"

# ELF_06

## What it does

How the binary works:
- Take a password as argument
- hash password
- compare to hardcoded hashed password

In the code we found the hashed password: "K+N.R.X+R1U-M,X%Q+U" (19 bytes)

hash function:
- Each byte of the password is hashed individually
- the way a byte is hashed depends in part on the size of the password
- there are a lot of collisions

## Solution

Reimplement hash function.

Brute force each byte of the password **one at the time**.

See solve.c for an implementation of the solution

One possible solution is "castatgaandjtrgppad".

# ELF_07

## Troubleshoot

error while loading shared libraries: libssl.so.1.1
-> https://stackoverflow.com/a/72633324

## What it does

takes a password as argument

checks password is 20 bytes long

checks password[0:3] == password[9:12]

password must be lowercase letters only

hashed = custom_hash(password) // same as ELF_06
hashed == ```K0!0!0!0(4&1\"1#0%6*0```

sha256(password[12:20]) == ```8531d8960e7f2447508d80e80d48fd96730cf89a9987268971d858fc49cba71a```

sha256(password[3:9]) == ```2053dbbf6ec7135c4e994d3464c478db6f48d3ca21052c8f44915edc96e02c39```

## Solution

See solve/src/main.rs for implementation.

### Checksums / sha256

for password[12:20] find all that match with custom_hash (around 900)
test sha256 for until one matches
-> we have password[12:20]
solution: stripped

do the same for password[3:9] (around 6750)
solution: static

Algo complexity for each character:
```
nb_checks(0) = 26
nb_checks(i) = 26 * valid_pwd(nb_checks(i))
```

### repeated parts

for password[0:3] find all that match with custom_hash (around 10)
test if they work when put on password[9:12]

2 solutions:
- not
- nat

### conclusion

2 solutions
- natstaticnatstripped
- notstaticnotstripped

# ELF_FAST_01

downolad ELF_FAST_01 twice.

## Step 1

by comparing .text section of the 2 we can find the position of the password.

## Step 2

Get the password using hardcoded indexes.

# ELF_FAST_02

downolad ELF_FAST_02 twice.

## Step 1

by comparing .text section of the 2 we can find the position of the password.

## Step 2

Get the password using hardcoded indexes.

## Step 3

Decrement each byte once.

# ELF_FAST_03

downolad ELF_FAST_03 twice.

## Step 1

by decompiling both we can see that secret is always the same.

## Step 2

by comparing .text section of the 2 we can find the position of the secret.

## Step 3

Get the secret using hardcoded indexes and apply the mask to get the password.

# QT_01

decompiled with cutter.

found flag in ```dbg.Notepad::save()```.

```c++
// WARNING: Variable defined which should be unmapped: var_8h
// WARNING: [rz-ghidra] Type QFile of variable file has size 0
// WARNING: [rz-ghidra] Type QTextStream of variable out has size 0
// WARNING: [rz-ghidra] Detected overlap for variable var_5ch
// WARNING: [rz-ghidra] Detected overlap for variable f2
// WARNING: [rz-ghidra] Detected overlap for variable var_ch
// WARNING: [rz-ghidra] Detected overlap for variable flags

void Notepad::save()(int64_t arg1)
{
    char cVar1;
    undefined4 uVar2;
    int64_t in_FS_OFFSET;
    undefined8 uVar3;
    Notepad *this;
    undefined4 var_5ch;
    QString fileName;
    QString text;
    int64_t var_48h;
    undefined auStack72 [16];
    undefined auStack56 [24];
    int64_t canary;
    int64_t var_8h;
    
    canary = *(int64_t *)(in_FS_OFFSET + 0x28);
    QString::QString()((int64_t)&fileName);
    cVar1 = QString::isEmpty() const(arg1 + 0x38);
    if (cVar1 == '\0') {
        QString::operator=(QString const&)(&fileName, arg1 + 0x38);
    } else {
        QFlags<QFileDialog::Option>::QFlags(int QFlags<QFileDialog::Option>::Private::*)((int64_t)&var_5ch, -1);
        QString::QString()((int64_t)auStack56);
        QString::QString()((int64_t)auStack72);
        uVar3 = 0x5d4a;
        QString::QString(char const*)((int64_t)&text, "Save");
        QFileDialog::getSaveFileName(QWidget*, QString const&, QString const&, QString const&, QString*, QFlags<QFileDialog::Option>)
                  (&var_48h, arg1, &text, auStack72, auStack56, 0, var_5ch, uVar3);
        QString::operator=(QString&&)((int64_t)&fileName, (int64_t)&var_48h);
        QString::~QString()((int64_t)&var_48h);
        QString::~QString()((int64_t)&text);
        QString::~QString()((int64_t)auStack72);
        QString::~QString()((int64_t)auStack56);
        QString::operator=(QString const&)(arg1 + 0x38, &fileName);
    }
    QFile::QFile(QString const&)(auStack72, &fileName);
    uVar2 = operator|(QIODevice::OpenModeFlag, QIODevice::OpenModeFlag)(2, 0x10);
    cVar1 = QFile::open(QFlags<QIODevice::OpenModeFlag>)(auStack72, uVar2);
    if (cVar1 == '\x01') {
        QWidget::setWindowTitle(QString const&)(arg1, &fileName);
        QTextStream::QTextStream(QIODevice*)(auStack56, auStack72);
        QTextEdit::toPlainText() const(&text, *(undefined8 *)(*(int64_t *)(arg1 + 0x30) + 0x90));
        QTextStream::operator<<(QString const&)(auStack56, &text);
        QFileDevice::close()(auStack72);
        QTextEdit::toPlainText() const(&var_48h, *(undefined8 *)(*(int64_t *)(arg1 + 0x30) + 0x90));
        cVar1 = QString::operator==(char const*) const((int64_t)&var_48h, (int64_t)"hack the world !");
        QString::~QString()((int64_t)&var_48h);
        if (cVar1 != '\0') {
            uVar3 = *(undefined8 *)(*(int64_t *)(arg1 + 0x30) + 0x90);
            QString::QString(char const*)((int64_t)&var_48h, "BFS[youpwndnotepad]"); // <---- HERE
            QTextEdit::setText(QString const&)(uVar3, &var_48h);
            QString::~QString()((int64_t)&var_48h);
        }
        QString::~QString()((int64_t)&text);
        QTextStream::~QTextStream()(auStack56);
    } else {
        QFlags<QMessageBox::StandardButton>::QFlags(QMessageBox::StandardButton)((int64_t)&var_5ch, 0x400);
        QIODevice::errorString() const(&text, auStack72);
        operator+(char const*, QString const&)((int64_t)&var_48h, "Cannot save file: ", (int64_t)&text);
        QString::QString(char const*)((int64_t)auStack56, "Warning");
        QMessageBox::warning(QWidget*, QString const&, QString const&, QFlags<QMessageBox::StandardButton>, QMessageBox::StandardButton)
                  (arg1, auStack56, &var_48h, var_5ch, 0);
        QString::~QString()((int64_t)auStack56);
        QString::~QString()((int64_t)&var_48h);
        QString::~QString()((int64_t)&text);
    }
    QFile::~QFile()(auStack72);
    QString::~QString()((int64_t)&fileName);
    if (canary != *(int64_t *)(in_FS_OFFSET + 0x28)) {
    // WARNING: Subroutine does not return
        __stack_chk_fail();
    }
    return;
}
```

# QT_02

Opened QtCalculator with cutter.

Decompiled (ghidra)

Found ```Calculator::check_result()```.

If the result of an operation is 4242421337.0, the flag will appear.

```c++
void Calculator::check_result()(int64_t arg1)
{
    undefined8 uVar1;
    int64_t in_FS_OFFSET;
    Calculator *this;
    int64_t var_40h;
    int64_t var_38h;
    int64_t var_30h;
    long int result;
    long int checksum;
    int64_t canary;
    
    canary = *(int64_t *)(in_FS_OFFSET + 0x28);
    result = 0;
    checksum = 0xfcde3659;
    if (*(double *)(arg1 + 0x38) == 4242421337.0) { // <----- Check is here
        QFlags<QMessageBox::StandardButton>::QFlags(QMessageBox::StandardButton)((int64_t)&var_40h, 0x400);
        QString::QString(char const*)((int64_t)&var_30h, " GG your treasure is now on the screen");
        QString::QString(char const*)((int64_t)&var_38h, "Warning");
        QMessageBox::warning(QWidget*, QString const&, QString const&, QFlags<QMessageBox::StandardButton>, QMessageBox::StandardButton)
                  (arg1, &var_38h, &var_30h, (undefined4)var_40h, 0);
        QString::~QString()((int64_t)&var_38h);
        QString::~QString()((int64_t)&var_30h);
        result = ((int64_t)(*(double *)(arg1 + 0x38) - 4200420037.0) + -0x280de5b) * 10000 + 0x539;
        Calculator::clearAll()(arg1);
        uVar1 = *(undefined8 *)(arg1 + 0x60);
        QString::number(long, int)(&var_40h, result, 10);
        operator+(char const*, QString const&)((int64_t)&var_38h, (int64_t)"BFS[", (int64_t)&var_40h);
        operator+(QString const&, char const*)((int64_t)&var_30h, (int64_t)&var_38h, 0x956c);
        QLineEdit::setText(QString const&)(uVar1, &var_30h);
        QString::~QString()((int64_t)&var_30h);
        QString::~QString()((int64_t)&var_38h);
        QString::~QString()((int64_t)&var_40h);
    }
    if (canary != *(int64_t *)(in_FS_OFFSET + 0x28)) {
    // WARNING: Subroutine does not return
        __stack_chk_fail();
    }
    return;
}
```

# RUST_01

## Solution

Honeslty just ran it with Cutter and then went into every function until the flag magically appeared in the stack.

Can be seen in ```dbg.main``` -> ```String::from_utf8``` -> ```dbg.index```

flag: ```BFS[nftzngcdotksrhzucjwaixiqsutfqwal]```

# HTTP

Run http bin with wireshark recording (filter by http requests)

(from http bin to server)\
http://reverse.blackfoot.io:5000/key/AtwPJxvbolRxyRyrD8Yp8wSd

(from server to http bin, in data)\
```0c13dbe52ea053034a84f33c25bbc4e20f4d88813407fba21b5c291ff8c452aa```

result:\
http://reverse.blackfoot.io:5000/validate/AtwPJxvbolRxyRyrD8Yp8wSd/0c13dbe52ea053034a84f33c25bbc4e20f4d88813407fba21b5c291ff8c452aa

# HTTPS

Decrypt HTTPS with wireshark: https://support.f5.com/csp/article/K50557518#OnLinux

Search for something that looks like this:
```
25  7.099731051 192.168.1.13    34.78.251.81    HTTP2   190 HEADERS[1]: GET /key/B42brPAq1KnMB3KVPfI27wtR
```

Get the second key:
```bash
curl https://keyvalidator.reverse.blackfoot.io/key/B42brPAq1KnMB3KVPfI27wtR
```

Get the flag:
```bash
curl https://keyvalidator.reverse.blackfoot.io/validate/B42brPAq1KnMB3KVPfI27wtR/b57ab317235d05d9615ee4053357bc700ad7e12e0d3b51ddbd94de04dd796ab5
```

# ez_breezy

open http://reverse.blackfoot.io:1330/ whith google chrome (firefox doesn't work).

go to **Debugger** and open index.wasm.

flag is in clear a the end of the file.

flag: ```ez_br33zy!```

```
(module
  (memory $a.memory (;0;) (import "a" "memory") 256 256)
  (global $global0 (mut i32) (i32.const 5244432))
  (func $verifyPassword (;0;) (export "b") (param $var0 i32) (result i32)
    local.get $var0
    call $strcmp
    i32.eqz
  )
  (func $strcmp (;1;) (param $var0 i32) (result i32)
    (local $var1 i32)
    (local $var2 i32)
    (local $var3 i32)
    i32.const 1024
    local.set $var1
    block $label0 (result i32)
      loop $label1
        local.get $var1
        i32.load8_s
        local.set $var2
        i32.const 0
        local.get $var0
        i32.load8_u
        local.tee $var3
        i32.eqz
        br_if $label0
        drop
        local.get $var2
        i32.const 255
        i32.and
        local.get $var3
        i32.eq
        if
          local.get $var1
          i32.const 1
          i32.add
          local.set $var1
          local.get $var0
          i32.const 1
          i32.add
          local.set $var0
          br $label1
        end
      end $label1
      local.get $var3
      i32.const 24
      i32.shl
      i32.const 24
      i32.shr_s
    end $label0
    local.get $var2
    i32.sub
  )
  (func $stackSave (;2;) (export "c") (result i32)
    global.get $global0
  )
  (func $stackRestore (;3;) (export "e") (param $var0 i32)
    local.get $var0
    global.set $global0
  )
  (func $stackAlloc (;4;) (export "d") (param $var0 i32) (result i32)
    global.get $global0
    local.get $var0
    i32.sub
    i32.const -16
    i32.and
    local.tee $var0
    global.set $global0
    local.get $var0
  )
  (func $__wasm_call_ctors (;5;) (export "a")
    nop
  )
  (data (i32.const 1024) "ez_br33zy!") <----- Flag
)
```

# KILL_LA_KOOL

Open http://reverse.blackfoot.io:1331/ with google chrome.\
Go to *Sources*.\
Open ```index.wasm```.

## What it does

When a password is entered it will be check by the verify password function.

It does some operations on ```!0#{W}R7_s30u__?_nyd```.

It compares the result with the password we gave it.

## Solution

Put a breakpoint on the line strcmp is called (```0x00ef```)

Run with the debugger until that line

The address to the modified password (the flag) will be in $var1, in Local.

Go to Module.memories.$a.memory

Click on the little icon next to it

It will open the memory inspector

Go to the address that was stored in ```$var1```, the flag will be there.

# ELF_antiDBG_01

opened with cutter

changed rax to 0 after ptrace call OR replace ptrace call with ```nop``` instruction.

found "lllloworldabcdef" in code, it goes through a compute() function to get the password.

Skip compute(), go to the strncmp and look at what the password is compared against.\
flag: "qgqgtrtmq_f]h_ja"

# ELF_antiDBG_02

## What the program does

When opened with a debugger it compares the password with '=sCc7_InB_9t5nB_D_C_D'.

This will not work when the program is run without a debugger.

The program uses the SIGTRAP handler technique: https://subscription.packtpub.com/book/networking-and-servers/9781782167105/4/ch04lvl1sec39/elf-anti-debugging-and-packing-techniques

Basically, it defines an handler for the SIGTRAP signal (called the program encounters a breakpoint) and has defined a breakpoint in its code. When run with a debugger, the debugger will overwrite the handler and the handler will not be called at the breakpoint.

The SIGTRAP handler hashes the given password with a custom hashing function, similarly to ELF_06 and ELF_07.

## Solution

Same as ELF_06, implement the hashing function and bruteforce 1 character at the time.

### Debugging

To debug the hash implementation, you can run the program with the debugger until the program-defined breakpoint and then change the RIP to the start of the SIGTRAP handler.

# PATCH_01

## What we know

The program needs to be patched.

We need a md5 of the file
-> The difference must be really small

## Brute force solution (bit flipping)

The original file is 13,756 bytes long.

Flip 1 bit in the file and see if it is the solution. Repeat for every bit in the file (110,048 attempts).

Took 7m33s to complete.

By flipping bit 4 of byte 4284 we get:
Received message: send_help_pls!

md5: ```1db328070f31c4c97c9569122ae2c729```

# OBFUSCATION_simple_binary

## Brute force

from the subject: "Note: Any password used by DOOM's henchmen can't be very long - they can only count to 3!"

with alphabet: 
```abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!#$%&()*+,-./:;<>=?@[]\\^_`{}|~```

we get almost 800,000 possible passwords

### Generate passwords

Used this program to generate all passwords.

```rust
fn n_permutations2<T>(l: &Vec<T>, n: usize) -> Vec<Vec<T>>
where
    T: Copy,
{
    let n_perms = l.len().pow(n as u32);
    let mut perms = Vec::with_capacity(n_perms);

    let n_values = l.len();
    let group_sizes: Vec<usize> = (0..n)
        .map(|depth| n_values.pow(n as u32 - depth as u32 - 1))
        .collect();

    for i in 0..n_perms {
        let mut perm = Vec::with_capacity(n);

        for depth in 0..n {
            let value = l[i / group_sizes[depth] % n_values];
            perm.push(value);
        }

        perms.push(perm);
    }

    perms
}

fn main() {
    let perms = n_permutations2(&"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!#$%&()*+,-./:;<>=?@[]\\^_`{}|~".chars().collect::<Vec<char>>(), 3);

    for c in perms {
        println!("{}", c.iter().collect::<String>());
    }
}
```

put result into file (\~1800 ms)

```
aaa
aab
aac
...
```

### Testing passwords

when testing passwords, there are 3 possible cases:
- 1: segfault (most likely result)
- 2: program exits with 0 and prints something (around 1/1000)
- 3: programs freezes (around 1/5000)

In case 2 the message usually contains unprintable characters and/or is unreadable.

Theory: when the right password is entered the program will print a readable message.

bash oneliner used to test:
```bash
for i in $(cat passwords.txt); do echo $i; res=$(./simple_binary $i); if [[ $? -eq 0 ]]; then echo '=================' >> logs.txt; echo $i >> logs.txt; echo $res >> logs.txt; fi; ; done
```

case 1: ignored\
case 2: message written to logs.txt, to be reviewed manually\
case 3: script stopped, passwords.txt is updated manually to remove everything up to where it froze (included)

### Result

It took 2 days, tested around 250,000 passwords.

password: "z0b" (lol)\
flag: ```BFS{5iMp1e_r1gH7? g00d J0b!}```

## Brute force v2

See ./solve/src/main.rs for the code.

Finds the flag in 2 hours.

# less simple binary

**Did not finish**

## What it does

Success message is 156 bytes long
