#[allow(unused)]
use std::collections::*;
use std::io::{BufRead, BufWriter, Write};

// const MOD:i64 = 1000000000+7;
#[allow(unused)]
pub fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();

    let mut h = vec![0usize; n];
    for i in 0..n {
        h[i] = scan.token::<usize>();
    }
    fn f(t: usize, h: &Vec<usize>) -> usize {
        let (mut a, mut b) = (0, 0);
        let mut ans = 0;
        for &v in h.iter() {
            a += (t - v) % 2;
            b += (t - v) / 2;
        }
        if a == b {
            ans = a * 2;
        } else if a > b {
            ans = a * 2 - 1;
        } else {
            ans = a * 2;
            b -= a;
            ans += b * 2 / 3 * 2;
            ans += b * 2 % 3;
        }
        return ans;
    }
    let &mx = h.iter().max().unwrap();

    writeln!(out, "{}", f(mx, &h).min(f(mx + 1, &h))).ok();
}

pub fn run(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let t = scan.token::<usize>();
    for _ in 0..t {
        solve(scan, out)
    }
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

// https://codeforces.com/contest/1661/problem/C
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步
