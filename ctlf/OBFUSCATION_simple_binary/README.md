# OBFUSCATION_simple_binary

## Brute force

from the subject: "Note: Any password used by DOOM's henchmen can't be very long - they can only count to 3!"

with alphabet: 
```abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!#$%&()*+,-./:;<>=?@[]\\^_`{}|~```

we get around 800,000 possible passwords

## Generate passwords

had this program lying around

```
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

put result into file

```
aaa
aab
aac
...
```

## Testing passwords

when testing passwords, 3 possible cases:
- 1: segfault (most likely result)
- 2: program exits with 0 and prints something (about 1/1000)
- 3: programs freezes (about 1/5000)

In case 2 the message usually contains unprintable characters and/or is unreadable

Theory: when the right password is entered the program will print a readable message

bash one liner used to test:
```
for i in $(cat passwords.txt); do echo $i; res=$(./simple_binary $i); if [[ $? -eq 0 ]]; then echo '=================' >> logs.txt; echo $i >> logs.txt; echo $res >> logs.txt; fi; ; done
```

case 1: ignored
case 2: message written to logs.txt, to be reviewed manually
case 3: script stopped, passwords.txt is updated manually to remove everything up to where it froze (included)

## Result

Took about 2 days
tested almost 300,000 passwords

real password: z0b (lol)
flag: BFS{5iMp1e_r1gH7? g00d J0b!}
