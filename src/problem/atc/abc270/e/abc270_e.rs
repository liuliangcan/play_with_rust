use itertools::Itertools;
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
}

// const MOD:i64 = 1000000000+7;
pub fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let mut n = scan.token::<usize>();
    let mut k = scan.token::<usize>();
    let mut a = vec![0usize; n];
    for i in 0..n {
        a[i] = scan.token::<usize>();
    }
    let mut b = a.clone();
    b.sort_unstable();
    let mut cnt = 0;
    for v in b {
        let d = v - cnt;
        if d > 0 {
            if k > d * n {
                k -= d * n;
                cnt = v;
            } else {
                cnt += k / n;
                k %= n;
                break;
            }
        }
        n -= 1;
    }
    for i in 0..a.len() {
        a[i] = if a[i] > cnt { a[i] - cnt } else { 0 };
        if a[i] > 0 && k > 0 {
            a[i] -= 1;
            k -= 1;
        }
    }
    writeln!(out, "{}", a.iter().join(" ")).ok();
}

pub fn run(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    solve(scan, out)
}
// https://atcoder.jp/contests/abc270/tasks/abc270_e
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步
