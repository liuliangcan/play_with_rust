use std::cmp::{max, min};
#[allow(unused)]
use std::collections::*;
use std::io::{BufRead, BufWriter, Write};
#[allow(unused)]
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
    //    pub fn token_bytes(&mut self) -> Vec<u8> {
    //        let s = self.token::<String>();
    //        return s.as_bytes().into();
    //    }
}

// const MOD:usize = 1000000000+7;
// https://codeforces.com/problemset/problem/1695/C
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步
#[allow(unused)]
pub fn solve1(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {  // 93ms
    let t = scan.token::<usize>();
    for _ in 0..t {
        let n = scan.token::<usize>();
        let m = scan.token::<usize>();

        let mut mx = vec![- 1000000000i64; m + 1];
        let mut mn = vec![1000000000i64; m + 1];
        mx[0] = 0;
        mn[0] = 0;
        for _ in 0..n {
            for j in 1..=m {
                let a = scan.token::<i64>();
                mx[j] = max(mx[j], mx[j - 1]) + a;
                mn[j] = min(mn[j], mn[j - 1]) + a;
            }
            mx[0] = -1000000000;
            mn[0] = 1000000000;
        }
        if (m + n) & 1 == 1 && mn[m] <= 0 && 0 <= mx[m] {
            writeln!(out, "YES").ok();
        } else {
            writeln!(out, "NO").ok();
        }
    }
}

pub fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let t = scan.token::<usize>();
    for _ in 0..t {
        let n = scan.token::<usize>();
        let m = scan.token::<usize>();

        let mut mx = vec![i32::MIN; m + 1];
        let mut mn = vec![i32::MAX; m + 1];
        mx[0] = 0;
        mn[0] = 0;
        for _ in 0..n {
            for j in 1..=m {
                let a = scan.token::<i32>();
                // mx[j] = max(mx[j], mx[j - 1]) + a;
                // mn[j] = min(mn[j], mn[j - 1]) + a;
                mx[j] = mx[j].max(mx[j - 1]) + a;
                mn[j] = mn[j].min(mn[j - 1]) + a;
            }
            mx[0] = i32::MIN;
            mn[0] = i32::MAX;
        }
        if (m + n) & 1 == 1 && mn[m] <= 0 && 0 <= mx[m] {
            writeln!(out, "YES").ok();
        } else {
            writeln!(out, "NO").ok();
        }
    }
}
