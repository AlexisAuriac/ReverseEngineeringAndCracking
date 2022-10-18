use sha2::{Digest, Sha256};

const TARGET_HASH: &str = "K0!0!0!0(4&1\"1#0%6*0";

const TARGET_SUM1: &str = "8531d8960e7f2447508d80e80d48fd96730cf89a9987268971d858fc49cba71a";
const TARGET_SUM2: &str = "2053dbbf6ec7135c4e994d3464c478db6f48d3ca21052c8f44915edc96e02c39";

const ALPHA: &str = "abcdefghijklmnopqrstuvwxyz";

fn hash(input: &str) -> String {
    let mut s = input.as_bytes().to_vec();
    let input_len = s.len();

    for i in 0..input_len {
        for j in 0..input_len {
            if j % 2 == 0 {
                if i % 2 == 0 {
                    s[j] = s[j].wrapping_add(0x1a);
                } else {
                    s[j] = s[j].wrapping_sub(0x2a);
                }
            } else {
                if i % 2 == 0 {
                    s[j] = s[j].wrapping_sub(3);
                } else {
                    s[j] = s[j].wrapping_sub(0x2a);
                }
            }

            if s[j] == 0 {
                s[j] = 0x2a;
            }
        }

        for j in 1..input_len {
            s[j] = s[j].wrapping_sub(2);

            if s[j] == 0 {
                s[j] = 0x2a;
            }
        }

        for j in 0..input_len {
            if s[j] > 127 {
                s[j] = !s[j] + 1;
            }

            if s[j] <= 0x20 {
                s[j] = s[j].wrapping_add(0x2f);
            }
        }
    }

    std::str::from_utf8(&s).unwrap().to_string()
}

fn partial_brute_force_custom_hash_all(
    target_hash: &str,
    offset: usize,
    size: usize,
) -> Vec<String> {
    let target_vec = target_hash.as_bytes();
    let target_len = target_vec.len();
    let mut stack = vec![(vec!['0' as u8; target_len], 0)];

    while stack[0].1 < size {
        let (pwd, i) = stack.remove(0);
        let mut pwd = pwd.clone();

        for c in ALPHA.chars() {
            pwd[offset + i] = c as u8;
            let hashed = hash(std::str::from_utf8(&pwd).unwrap());

            if hashed[offset..(offset + i + 1)].eq(&target_hash[offset..(offset + i + 1)]) {
                stack.push((pwd.clone(), i + 1));
            }
        }
    }

    stack
        .into_iter()
        .map(|(pwd, _i)| std::str::from_utf8(&pwd).unwrap().to_string())
        .collect()
}

fn brute_force_checksum(target_hash: &str, target_sum: &str, offset: usize, size: usize) -> String {
    let pwds = partial_brute_force_custom_hash_all(target_hash, offset, size);

    let mut hasher = Sha256::new();
    for pwd in &pwds {
        hasher.update(&pwd[offset..(offset + size)]);

        let result = hasher.finalize_reset();
        let result = result
            .into_iter()
            .map(|x| format!("{:02x}", x))
            .collect::<Vec<String>>()
            .join("");

        if result.eq(target_sum) {
            return pwd[offset..(offset + size)].to_string();
        }
    }

    panic!("Did not find match");
}

fn brute_force_repeated_part() -> Vec<String> {
    partial_brute_force_custom_hash_all(TARGET_HASH, 0, 3)
        .into_iter()
        .filter(|pwd_part| {
            let pwd = format!("000000000{}00000000", &pwd_part[..3]);
            let hashed = hash(&pwd);

            hashed[9..12].eq(&TARGET_HASH[9..12])
        })
        .map(|pwd_part| pwd_part[..3].to_string())
        .collect()
}

fn main() {
    let pwd_checksum1 = brute_force_checksum(TARGET_HASH, TARGET_SUM1, 12, 8);
    let pwd_checksum2 = brute_force_checksum(TARGET_HASH, TARGET_SUM2, 3, 6);
    let pwd_repeated = brute_force_repeated_part();

    let valid_pwd = pwd_repeated
        .into_iter()
        .map(|repeated| format!("{}{}{}{}", repeated, pwd_checksum2, repeated, pwd_checksum1))
        .collect::<Vec<_>>();

    println!("Possible passwords:");
    for pwd in valid_pwd {
        println!("{}", pwd);
    }
}
