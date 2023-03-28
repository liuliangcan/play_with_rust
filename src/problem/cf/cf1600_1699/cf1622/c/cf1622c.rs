#[allow(unused)]
use std::collections::*;
use std::io::{BufRead, BufWriter, Write};

// const MOD:i64 = 1000000000+7;
#[allow(unused)]
pub fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let k = scan.token::<usize>();
    let mut a = vec![0; n];
    for i in 0..n {
        a[i] = scan.token::<usize>();
    }
    let s: usize = a.iter().sum();
    if s <= k {
        writeln!(out, "0").ok();
        return;
    }
    if a.len() == 1 {
        writeln!(out, "{}", s - k).ok();
        return;
    }
    a.sort_unstable();
    let d = s - k;
    let mut ans = d;
    let mut s = 0;
    let mut cnt = 0;
    let l = a[0];
    for i in (1..n).rev() {
        s += a[i];
        cnt += 1;
        if s - l * cnt >= d {
            ans = ans.min(cnt);
            break;
        }
        let x = (l * cnt + d + cnt + 1 - 1 - s) / (cnt + 1);
        ans = ans.min(x + cnt);
    }
    writeln!(out, "{}", ans).ok();
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

// https://codeforces.com/contest/1622/problem/C
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步
