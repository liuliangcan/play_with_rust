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
    #[allow(unused)]
    pub fn token_bytes(&mut self) -> Vec<u8> {
        let s = self.token::<String>();
        return s.as_bytes().into();
    }
}

// const MOD:i64 = 1000000000+7;
pub fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let mut s = scan.token_bytes();
    s.sort_unstable();
    let mut g = vec![Vec::new(); 2];
    for c in s {
        g[c as usize & 1].push(c);
    }
    if g[0].is_empty() {
        writeln!(out, "{}", String::from_utf8_lossy(&g[1])).ok();
    } else if g[1].is_empty() {
        writeln!(out, "{}", String::from_utf8_lossy(&g[0])).ok();
    } else if g[0].last().unwrap().abs_diff(g[1][0]) != 1 {
        writeln!(
            out,
            "{}{}",
            String::from_utf8_lossy(&g[0]),
            String::from_utf8_lossy(&g[1])
        )
        .ok();
    } else if g[1].last().unwrap().abs_diff(g[0][0]) != 1 {
        writeln!(
            out,
            "{}{}",
            String::from_utf8_lossy(&g[1]),
            String::from_utf8_lossy(&g[0])
        )
        .ok();
    } else {
        writeln!(out, "No answer").ok();
    }
}

pub fn run(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let t = scan.token::<usize>();
    for _ in 0..t {
        solve(scan, out)
    }
}
// http://codeforces.com/problemset/problem/1156/B
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步
