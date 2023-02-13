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
    //    pub fn token_bytes(&mut self) -> Vec<u8> {
    //        let s = self.token::<String>();
    //        return s.as_bytes().into();
    //    }
}

// const MOD:i64 = 1000000000+7;
pub fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let h = scan.token::<usize>();
    let l = scan.token::<usize>();
    let r = scan.token::<usize>();
    let mut a = vec![0i32; n];
    for i in 0..n {
        a[i] = scan.token::<i32>();
    }
    let mut f = vec![vec![0i32; h]; n + 1];
    for s in l..=r {
        f[n][s] = 1
    }
    for i in (0..n).rev() {
        for s in 0..h {
            f[i][s] = f[i + 1][((s + a[i] as usize) % h) as usize]
                .max(f[i + 1][((s + a[i] as usize - 1) % h) as usize]);
            if i > 0 && l <= s && s <= r {
                f[i][s] += 1
            }
        }
    }
    writeln!(out, "{}", f[0][0]).ok();
}

pub fn run(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    solve(scan, out)
}
// https://codeforces.com/contest/1324/problem/E
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步
