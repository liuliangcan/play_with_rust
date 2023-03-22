#[allow(unused)]
use std::collections::*;
use std::io::{BufRead, BufWriter, Write};

// const MOD:i64 = 1000000000+7;
#[allow(unused)]
pub fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let m = scan.token::<usize>();
    let mut g = Vec::new();
    for _ in 0..n {
        g.push(scan.token_bytes());
    }
    let mut left = vec![1; n];
    let mut ans = 0;
    for j in 0..m {
        if j > 0 {
            for i in 0..n {
                if g[i][j] == g[i][j - 1] {
                    left[i] += 1;
                } else {
                    left[i] = 1;
                }
            }
        }
        let mut up = vec![1; n];
        let mut f1 = left.clone();
        let mut f2 = left.clone();
        for i in (0..n - 1).rev() {
            if g[i][j] == g[i + 1][j] {
                f2[i] = f2[i].min(f2[i + 1])
            }
        }
        for i in 1..n {
            if g[i][j] == g[i - 1][j] {
                f1[i] = f1[i].min(f1[i - 1]);
                up[i] += up[i - 1]
            }
            let u = up[i];
            if i + 1 < u * 3 || up[i - u] != u || up[i - u * 2] < u {
                continue;
            }
            ans += f1[i].min(f1[i - u]).min(f2[i + 1 - 3 * u])
        }
    }
    writeln!(out, "{}", ans).ok();
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

// https://codeforces.com/problemset/problem/1181/C
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步
