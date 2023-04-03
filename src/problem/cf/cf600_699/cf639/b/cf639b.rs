#[allow(unused)]
use std::collections::*;
use std::io::{BufRead, BufWriter, Write};

// const MOD:i64 = 1000000000+7;
#[allow(unused)]
pub fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let d = scan.token::<usize>();
    let h = scan.token::<usize>();
    if 2*h <d || d==1&&n>2{
        writeln!(out, "-1").ok();
        return;
    }
    if n == 2{
        writeln!(out,"1 2").ok();
        return;
    }
    if h == 1{
        for i in 2..=n{
            writeln!(out,"1 {}",i).ok();
        }
        return
    }
    for i in 1..=h{
        writeln!(out,"{} {}",i,i+1).ok();
    }
    if d > h{
        writeln!(out,"1 {}",h+2).ok();
        for i in h+2..=d{
            writeln!(out,"{} {}",i,i+1).ok();
        }
    }
    for i in d+2..=n{
        writeln!(out,"2 {}",i).ok();
    }
}

pub fn run(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {

    solve(scan,out)
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

// https://codeforces.com/problemset/problem/639/B
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步
