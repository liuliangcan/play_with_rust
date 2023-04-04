#[allow(unused)]
use std::collections::*;
use std::io::{BufRead, BufWriter, Write};

const MOD: usize = 1000000000 + 7;
fn perm(a: usize, b: usize) -> usize {
    if a < b {
        return 0;
    }
    let mut ans = 1;
    for i in a - b + 1..=a {
        ans = ans * i % MOD
    }
    return ans;
}
#[allow(unused)]
pub fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let x = scan.token::<usize>();
    let pos = scan.token::<usize>();
    let (mut l, mut r) = (0, n);
    let (mut less, mut greater) = (0, 0);
    while l < r {
        let mid = (l + r) / 2;
        if mid < pos {
            less += 1;
            l = mid + 1;
        } else if mid > pos {
            greater += 1;
            r = mid;
        } else {
            l = mid + 1
        }
    }
    writeln!(
        out,
        "{}",
        perm(x - 1, less) * perm(n - x, greater) % MOD
            * perm(n - less - greater - 1, n - less - greater - 1)
            % MOD
    )
    .ok();
}

pub fn run(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    solve(scan, out)
}

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
    #[allow(unused)]
    pub fn token_bytes(&mut self) -> Vec<u8> {
        let s = self.token::<String>();
        return s.as_bytes().into();
    }
}

// https://codeforces.com/problemset/problem/1436/C
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步
