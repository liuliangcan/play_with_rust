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


pub fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let t = scan.token::<usize>();
    let s1 = scan.token_bytes();
    let s2 = scan.token_bytes();
    let p = n - t;
    let mut same = vec![0usize;0];
    let mut diff = vec![0usize;0];
    for (i,&c) in s1.iter().enumerate() {
        if c == s2[i] {
            same.push(i);
        }
        else {
            diff.push(i);
        }
    }
    let s = same.len();
    if p > s && (p-s)*2 > diff.len() {
        writeln!(out, "-1").ok();
        return;
    }
    let mut ans = vec![0u8;n];
    for i in 0..p.min(s) {
        ans[same[i]] = s1[same[i]];
    }
    if p > s{
        for i in 0..p-s{
            ans[diff[i*2]] = s1[diff[i*2]];
            ans[diff[i*2+1]] = s2[diff[i*2+1]];
        }
    }
    let abc = [b'a', b'b', b'c'];
    for i in 0..n {
        if ans[i] == 0 {
            for &d in abc.iter() {
                if d != s1[i] && d!= s2[i] {
                    ans[i] = d;
                    break;
                }
            }
        }
    }
    writeln!(out, "{}", String::from_utf8_lossy(&ans)).ok();
    // writeln!(out, "{}", String::from_utf8(ans).unwrap()).ok();
}

pub fn run(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    solve(scan,out)
}
// https://codeforces.com/problemset/problem/584/C
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步
