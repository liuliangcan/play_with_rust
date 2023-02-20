#[allow(unused)]
use std::collections::*;
use std::io::{BufRead, BufWriter, Write};
#[allow(unused)]
fn main() {
    let sin = std::io::stdin();
    let scan = &mut Scanner::new(sin.lock());
    let sout = std::io::stdout();
    let out = &mut BufWriter::new(sout.lock());
    run(scan, out);
}

pub struct Scanner<R> {
    reader: R,
    buffer: Vec<String>,
}
impl<R: ::std::io::BufRead> Scanner<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buffer: vec![],
        }
    }
    pub fn token<T: ::std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed parse");
            }
            let mut input = String::new();
            self.reader.read_line(&mut input).expect("Failed read");
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }
    pub fn token_bytes(&mut self) -> Vec<u8> {
        let s = self.token::<String>();
        return s.as_bytes().into();
    }
}

// const MOD:i64 = 1000000000+7;
pub fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let s = scan.token_bytes();
    let mut cnt = vec![0usize; 26];
    for c in s {
        cnt[(c - b'a') as usize] += 1;
    }
    let (mut l, mut r) = (0usize, 25usize);
    while l < r {
        while l < r && cnt[l] % 2 == 0 {
            l += 1;
        }
        while l < r && cnt[r] % 2 == 0 {
            r -= 1;
        }
        if l < r {
            cnt[l] += 1;
            cnt[r] -= 1;
            l += 1;
            r -= 1;
        }
    }
    let mut mid = '\0';
    for i in 0..26 {
        for _ in 0..(cnt[i] / 2) {
            write!(out, "{}", (i as u8 + b'a') as char).ok();
        }
        if cnt[i] % 2 == 1 {
            mid = (i as u8 + b'a') as char
        }
    }
    if mid != '\0' {
        write!(out, "{}", mid).ok();
    }

    for i in (0..26).rev() {
        for _ in 0..(cnt[i] / 2) {
            write!(out, "{}", (i as u8 + b'a') as char).ok();
        }
    }
    write!(out, "\n").ok();
}
#[allow(unused)]
pub fn solve1(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let s = scan.token_bytes();
    let mut cnt = vec![0usize; 26];
    for c in s {
        cnt[(c - ('a' as u8)) as usize] += 1;
    }
    let (mut l, mut r) = (0usize, 25usize);
    while l < r {
        while l < r && cnt[l] % 2 == 0 {
            l += 1;
        }
        while l < r && cnt[r] % 2 == 0 {
            r -= 1;
        }
        if l < r {
            cnt[l] += 1;
            cnt[r] -= 1;
            l += 1;
            r -= 1;
        }
    }
    let mut mid = '\0';
    for i in 0..26 {
        for _ in 0..(cnt[i] / 2) {
            write!(out, "{}", (i as u8 + 'a' as u8) as char).ok();
        }
        if cnt[i] % 2 == 1 {
            mid = (i as u8 + 'a' as u8) as char
        }
    }
    if mid != '\0' {
        write!(out, "{}", mid).ok();
    }

    for i in (0..26).rev() {
        for _ in 0..(cnt[i] / 2) {
            write!(out, "{}", (i as u8 + 'a' as u8) as char).ok();
        }
    }
    write!(out, "\n").ok();
}

pub fn run(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    solve(scan, out)
}
// https://codeforces.com/problemset/problem/600/C
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步
