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

pub fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let mut s = scan.token_bytes();
    s.insert(0, b')');
    let n = s.len();
    let mut f = vec![0usize; n];
    let mut ans = 0;
    let mut cnt = 1;
    for i in 2..n {
        if s[i] == b')' {
            if s[i - 1] == b'(' {
                f[i] = f[i - 2] + 2
            } else if f[i - 1] > 0 && s[i - f[i - 1] - 1] == b'(' {
                f[i] = f[i - 1] + 2 + f[i - f[i - 1] - 2]
            }
            if ans < f[i] {
                ans = f[i];
                cnt = 1;
            } else if ans == f[i] && ans > 0 {
                cnt += 1;
            }
        }
    }
    writeln!(out, "{} {}", ans, cnt).ok();
}

pub fn run(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    solve(scan, out)
}
// https://codeforces.com/problemset/problem/5/C
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步
