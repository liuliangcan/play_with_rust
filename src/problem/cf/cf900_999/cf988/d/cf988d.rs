#[allow(unused)]
use std::collections::*;
use std::io::{BufRead, BufWriter, Write};

// const MOD:i64 = 1000000000+7;
#[allow(unused)]
pub fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let mut a = vec![0; n];
    for i in 0..n {
        a[i] = scan.token::<i64>();
    }
    let &mx = a.iter().max().unwrap();
    let mut ans = vec![a[0]];
    let s: HashSet<_> = HashSet::from_iter(a);
    for &x in s.iter() {
        for k in 0..=32 {
            let t = x + (1 << k);
            if t > mx {
                break;
            }
            if s.contains(&t) {
                ans = vec![x, t];
                if s.contains(&(x + (1 << (k + 1)))) {
                    writeln!(out, "3\n{} {} {}", x, t, x + (1 << (k + 1))).ok();
                    return;
                }
            }
        }
    }
    writeln!(out, "{}", ans.len()).ok();
    for v in ans.iter() {
        writeln!(out, "{} ", v).ok();
    }
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

// https://codeforces.com/contest/988/problem/D
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步
