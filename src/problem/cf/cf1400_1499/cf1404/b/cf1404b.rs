#[allow(unused)]
use std::collections::*;
use std::io::{BufRead, BufWriter, Write};

// const MOD:i64 = 1000000000+7;
#[allow(unused)]
pub fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let a = scan.token::<usize>() - 1;
    let b = scan.token::<usize>() - 1;
    let da = scan.token::<usize>();
    let db = scan.token::<usize>();
    let mut g = vec![Vec::new(); n];
    for _ in 0..n - 1 {
        let u = scan.token::<usize>() - 1;
        let v = scan.token::<usize>() - 1;
        g[u].push(v);
        g[v].push(u);
    }
    if da * 2 >= db {
        writeln!(out, "Alice").ok();
        return;
    }
    let mut d = 0;
    let mut ans = 0; // ab距离
    let mut start = a;
    let mut q = VecDeque::new();
    q.push_back((a, n));
    while !q.is_empty() {
        d += 1;
        for _ in 0..q.len() {
            let (u, fa) = q.pop_front().unwrap();
            for &v in g[u].iter() {
                 if v == fa {
                    continue;
                }
                if v == b {
                    ans = d;
                }
                start = v;
                q.push_back((v, u));
            }
        }
    }
    if ans <= da {
        writeln!(out, "Alice").ok();
        return;
    }
    q.push_back((start, n));
    d = 0;
    while !q.is_empty() {
        d += 1;
        for _ in 0..q.len() {
            let (u, fa) = q.pop_front().unwrap();
            for &v in g[u].iter() {
                if v == fa {
                    continue;
                }
                q.push_back((v, u));
            }
        }
    }
    if d - 1 <= da * 2 {
        writeln!(out, "Alice").ok();
        return;
    }
    writeln!(out, "Bob").ok();
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

// https://codeforces.com/problemset/problem/1404/B
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步
