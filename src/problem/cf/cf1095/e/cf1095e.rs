#[allow(unused)]
use std::collections::*;
use std::io::{BufRead, BufWriter, Write};

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
// s作为 u8数组(bytes)读入 31ms
pub fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let s = scan.token_bytes();
    if n & 1 == 1 {
        writeln!(out, "0",).ok();
        return;
    }
    let mut suf = vec![(n * 2) as i32; n + 1];
    suf[n] = 0;
    let l = '(' as u8;
    let r = ')' as u8;
    for j in (0..n).rev() {
        if s[j] == r {
            suf[j] = suf[j + 1] + 1;
        } else {
            suf[j] = suf[j + 1] - 1;
            if suf[j] < 0 {
                break;
            }
        }
    }
    let mut ans = 0;
    let mut p = 0i32;
    for i in 0..n {
        if s[i] == l {
            if p > 0 && p == suf[i + 1] + 1 {
                ans += 1;
            }
            p += 1;
        } else {
            if suf[i + 1] > 0 && p + 1 == suf[i + 1] {
                ans += 1;
            }
            p -= 1;
            if p < 0 {
                break;
            }
        }
    }
    writeln!(out, "{}", ans).ok();
}

// s作为String读入后转chars 31ms
pub fn solve2(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let s = scan.token::<String>().chars().collect::<Vec<_>>();
    if n & 1 == 1 {
        writeln!(out, "0",).ok();
        return;
    }
    let mut suf = vec![(n * 2) as i32; n + 1];
    suf[n] = 0;
    for j in (0..n).rev() {
        if s[j] == ')' {
            suf[j] = suf[j + 1] + 1;
        } else {
            suf[j] = suf[j + 1] - 1;
            if suf[j] < 0 {
                break;
            }
        }
    }
    let mut ans = 0;
    let mut p = 0i32;
    for i in 0..n {
        if s[i] == '(' {
            if p > 0 && p == suf[i + 1] + 1 {
                ans += 1;
            }
            p += 1;
        } else {
            if suf[i + 1] > 0 && p + 1 == suf[i + 1] {
                ans += 1;
            }
            p -= 1;
            if p < 0 {
                break;
            }
        }
    }
    writeln!(out, "{}", ans).ok();
}

// s作为String读入后切片&str处理
pub fn solve1(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let s = scan.token::<String>();
    if n & 1 == 1 {
        writeln!(out, "0",).ok();
        return;
    }
    let mut suf = vec![(n * 2) as i32; n + 1];
    suf[n] = 0;
    for j in (0..n).rev() {
        if &s[j..j + 1] == ")" {
            suf[j] = suf[j + 1] + 1;
        } else {
            suf[j] = suf[j + 1] - 1;
            if suf[j] < 0 {
                break;
            }
        }
    }
    let mut ans = 0;
    let mut p = 0i32;
    for i in 0..n {
        if &s[i..i + 1] == "(" {
            if p > 0 && p == suf[i + 1] + 1 {
                ans += 1;
            }
            p += 1;
        } else {
            if suf[i + 1] > 0 && p + 1 == suf[i + 1] {
                ans += 1;
            }
            p -= 1;
            if p < 0 {
                break;
            }
        }
    }
    writeln!(out, "{}", ans).ok();
}

pub fn run(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    solve(scan, out)
}
// https://codeforces.com/problemset/problem/1095/E
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步
