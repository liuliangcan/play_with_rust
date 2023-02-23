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
    let n = scan.token::<usize>();
    let k = scan.token::<usize>();
    let mut a = vec![0usize; n];
    for i in 0..n {
        a[i] = scan.token::<usize>();
    }
    for _ in 0..n {
        scan.token::<i32>();
    }
    a.sort_unstable();
    let mut f = vec![0usize; n + 1]; // f[i]代表a[:i]里一个k最大覆盖多少，这个可以滑窗搞定
    let mut ans = 0;
    let mut l = 0usize;
    for (i, &v) in a.iter().enumerate() {
        while v - a[l] > k {
            l += 1;
        }
        f[i + 1] = f[i].max(i - l + 1);
        ans = ans.max(i - l + 1 + f[l]);  // 滑窗同时加上窗左边的最大窗，更新答案
    }
    writeln!(out, "{}", ans).ok();
}

pub fn run(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let t = scan.token::<usize>();
    for _ in 0..t {
        solve(scan, out)
    }
}
// https://codeforces.com/problemset/problem/1409/E
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步
