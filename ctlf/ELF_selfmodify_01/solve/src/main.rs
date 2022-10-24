use sha2::{Digest, Sha256};
use std::{fmt::Write, num::ParseIntError};

// https://stackoverflow.com/a/52992629/12864941
fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

fn encode_hex(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        write!(&mut s, "{:02x}", b).unwrap();
    }
    s
}

const TARGET_HASH: &str = "1b1d7478ccdc83c23d897948e110db093459481e1a3b20e5cf85dfe3f69f6804";

// const TARGET_HASH: &str = "7d56cf902da568f8395289d03ef484bba31dae4d2842dbc7bee6699dcaea95d6";
// const TARGET_HASH: &str = "a3015b1a5d5a754be06e6a1d3b1609a6df7fbdc5e1d02da27d0f2f0154d97826";
// const TARGET_HASH: &str = "7109925175dc4aadb75862fac9df4b2a81b7da71ae230cebf675a95a4568650c";
// const TARGET_HASH: &str = "935a02e5ccec99277a2542a403ad765c38448f56e20d0b349c50c5fc4c36cb1c";
// 1863b972f7490d03a6db27bca0083720
// 415c0db4902cf5fde8f8f6254a0f3d95
// ea7290655f6eccba23bad2b452c2b145
// 09243027beaafc687e4ee41773c14e63&W

fn n_permutations(
    l: &Vec<char>,
    n: usize,
    offset: usize,
) -> impl Iterator<Item = (usize, String)> + '_ {
    let n_perms = l.len().pow(n as u32);

    let n_values = l.len();
    let group_sizes: Vec<usize> = (0..n)
        .map(|depth| n_values.pow(n as u32 - depth as u32 - 1))
        .collect();

    (offset..n_perms).map(move |i| {
        let mut perm = String::with_capacity(n);

        for depth in 0..n {
            let value = l[i / group_sizes[depth] % n_values];
            perm.push(value);
        }

        (i, perm)
    })
}

fn main() {
    let target_hash = decode_hex(TARGET_HASH).unwrap();
    let alpha = &"abcdefghijklmnopqrstuvwxyz".chars().collect::<Vec<char>>();
    let perms = n_permutations(alpha, 8, 25003000000);

    let mut hasher = Sha256::new();

    for (i, pwd) in perms {
        let pwd = std::iter::repeat(pwd).take(2).collect::<String>();

        hasher.update(&pwd);
        let result = hasher.finalize_reset();

        // println!("=====================");
        // println!("{}", pwd);
        // println!("{}", encode_hex(&result));

        if result[..] == target_hash {
            println!("{}", pwd);
            return;
        }

        if i % 1_000_000 == 0 {
            println!("{}", i);
        }
    }
}
