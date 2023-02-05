#[allow(unused)]
use std::collections::*;
use std::io::{BufRead, BufWriter, Write};

fn main() {
    let sin = std::io::stdin();
    let scan = &mut Scanner::new(sin.lock());
    let sout = std::io::stdout();
    let out = &mut BufWriter::new(sout.lock());
    solve(scan, out);
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


// const MOD:usize = 1000000000+7;
// https://atcoder.jp/contests/abc272/tasks/abc272_e
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步
pub fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let m = scan.token::<usize>();
    let mut b = vec![vec![]; m + 1];

    for i in 1..=n {
        let v = scan.token::<i64>();
        let mut l = 1;
        if v < 0 {
            l = ((-v) as usize + i - 1) / i;
        }
        for j in l..=m {
            let p = (v + (i * j) as i64) as usize;
            if p < n {
                b[j].push(p)
            } else {
                break;
            }
        }
    }

    let mut t = vec![0usize; n + 1];
    for j in 1..=m {
        for &v in &(b[j]) {
            t[v] = j;
        }
        let mut mex = 0;
        while t[mex] == j {
            mex += 1
        }
        writeln!(out, "{}", mex).ok();
    }
}
