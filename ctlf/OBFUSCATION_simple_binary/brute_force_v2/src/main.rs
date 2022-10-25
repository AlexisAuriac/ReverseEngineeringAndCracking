use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use wait_timeout::ChildExt;

const ALPHA: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
// const ALPHA: &str = "bz019a";

const PROGRAM: &str = "../simple_binary";
const PROC_TIMEOUT: Duration = Duration::from_secs(1);

const PWD_IDX_CACHE: &str = "pwd_idx_cache.txt";
const SAVE_INTERVAL: u128 = 2000;

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

fn try_pwd(pwd: &str) -> Option<String> {
    let mut proc = Command::new(PROGRAM)
        .arg(pwd)
        .stdout(Stdio::piped())
        .spawn()
        .expect("Could not run command");

    match proc.wait_timeout(PROC_TIMEOUT) {
        Ok(None) => {
            proc.kill().unwrap();
            None
        }
        Ok(Some(status)) if status.success() => {
            let mut output = String::new();

            if proc.stdout.unwrap().read_to_string(&mut output).is_ok() {
                Some(output)
            } else {
                None
            }
        }
        _ => None,
    }
}

fn log_possible_solution(logs: &mut File, pwd: &str, res: &str, i: usize) -> std::io::Result<()> {
    writeln!(logs, "=================")?;
    writeln!(logs, "{}", i)?;
    writeln!(logs, "{}", pwd)?;
    writeln!(logs, "{}", res)
}

fn save_pwd_idx(i: usize) -> std::io::Result<()> {
    let mut cache = OpenOptions::new()
        .create(true)
        .write(true)
        .open(PWD_IDX_CACHE)?;

    write!(cache, "{}", i)
}

fn load_pwd_idx() -> std::io::Result<usize> {
    if Path::new(PWD_IDX_CACHE).exists() {
        let s = std::fs::read_to_string(PWD_IDX_CACHE)?;

        Ok(s.parse().expect("Invalid cache"))
    } else {
        Ok(0)
    }
}

fn main() -> std::io::Result<()> {
    let mut logs = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open("logs.txt")?;
    let pwd_offset = load_pwd_idx()?;

    let alpha = &ALPHA.chars().collect::<Vec<char>>();
    let perms = n_permutations(alpha, 3, pwd_offset);

    let mut last_save = Instant::now();

    for (i, pwd) in perms {
        if let Some(res) = try_pwd(&pwd) {
            log_possible_solution(&mut logs, &pwd, &res, i)?;
        }

        if last_save.elapsed().as_millis() > SAVE_INTERVAL {
            save_pwd_idx(i)?;
            last_save = Instant::now();
        }
    }

    Ok(())
}
