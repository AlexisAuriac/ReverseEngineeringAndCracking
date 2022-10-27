# ELF_selfmodify_01

## What it does

changes the protection of a part of the program so that it can be written onto (mprotect), this part contains 2 functions with a lot of invalid instructions

xors this section with 0x42, patching all errors

checks pwd is 16 bytes long

checks pwd is lowercase only

hash pwd with sha256

compares with: 1b1d7478ccdc83c23d897948e110db093459481e1a3b20e5cf85dfe3f69f6804

## Brute force (bad solution)

There is the a string "flagflagflagflag".

It doesn't seem to be used but could be a hint that the flag is a 4 byte string repeated for times.

Only 26^4 possibilities (456,976).

Bruteforcing it disproved that possibility.

(See solve/src/main.rs)

## Solution

Used an online hash db: https://hashes.com/en/decrypt/hash

flag: flatearthsociety
