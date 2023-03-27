#[allow(unused)]
use std::collections::*;
use std::io::{BufRead, BufWriter, Write};

// const MOD:i64 = 1000000000+7;
#[allow(unused)]
pub fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let mut a = vec![0i64; n];
    let mut st = Vec::new();
    let mut pre = Vec::new();
    let mut s = 0;
    let mut flag = true;
    for i in 0..n {
        a[i] = scan.token::<i64>();
        while !st.is_empty() && *st.last().unwrap() <= a[i] {
            if *pre.last().unwrap() < s {
                flag = false;
            }
            st.pop();
            pre.pop();
        }
        st.push(a[i]);
        pre.push(s);
        s += a[i];
    }
    if !flag {
        writeln!(out, "NO").ok();
        return;
    }
    let mut st = Vec::new();
    let mut pre = Vec::new();
    let mut s = 0;
    for &v in a.iter().rev() {
        while !st.is_empty() && *st.last().unwrap() <= v {
            if *pre.last().unwrap() < s {
                writeln!(out, "NO").ok();
                return;
            }
            st.pop();
            pre.pop();
        }
        st.push(v);
        pre.push(s);
        s += v;
    }
    writeln!(out, "YES").ok();
    return;
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

// https://codeforces.com/problemset/problem/1691/D
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步
