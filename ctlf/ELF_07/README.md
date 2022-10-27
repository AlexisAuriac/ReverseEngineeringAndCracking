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
