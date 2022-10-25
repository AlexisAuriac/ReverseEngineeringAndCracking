# ELF_selfmodify_01

## What it does

changes the protection of a part of the program so that it can be written onto (mprotect), this part contains 2 functions with a lot of invalid instructions

xors this section with 0x42, patching all errors

checks pwd is 16 bytes long

checks pwd is lowercase only

hash pwd with sha256

compares with: 1b1d7478ccdc83c23d897948e110db093459481e1a3b20e5cf85dfe3f69f6804

## Solution

https://hashes.com/en/decrypt/hash
flag: flatearthsociety
