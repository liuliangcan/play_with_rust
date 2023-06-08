#[allow(unused)]
use std::collections::*;
use std::io::{BufRead, BufWriter, Write};

const MOD: usize = 1000000000 + 7;

pub struct Program {
    // n: usize,
    // clock: usize,
    // a: Vec<usize>,
    // ans: Vec<usize>,
    // time: Vec<usize>,
}
impl Program {
    pub fn new() -> Self {
        Self {
            // n: 0,
            // clock: 0,
            // a: Vec::new(),
            // ans: Vec::new(),
            // time: Vec::new(),
        }
    }
    #[allow(unused)]
    pub fn solve(&mut self, scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
        let mut n = scan.token::<usize>();
        let mut a = scan.token::<usize>();
        let mut b = scan.token::<usize>();
        let mut k = scan.token::<usize>();
        let mut f = vec![1; n + 1];
        f[b] = 0;

        for i in 0..k {
            let mut g = vec![0; n + 1];
            let mut p = vec![0; n + 2];
            for (i, &v) in f.iter().enumerate() {
                p[i + 1] = (p[i] + v) % MOD;
            }

            for j in 1..=n {
                if j == b {
                    continue;
                }
                let d = b.abs_diff(j);
                let l = if j > d { j - d + 1 } else { 1 };
                let r = if j + d - 1 < n { j + d - 1 } else { n };

                g[j] = (p[r + 1] + MOD + MOD - p[l] - f[j]) % MOD;
            }
            f = g
        }

        writeln!(out, "{}", (f[a] + MOD) % MOD).ok();
    }
}
pub fn run(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let mut p = Program::new();

    // for _ in 0..scan.token::<usize>(){
    //     p.solve(scan, out)
    // }
    p.solve(scan, out)
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
// https://codeforces.com/contest/786/problem/C
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步
