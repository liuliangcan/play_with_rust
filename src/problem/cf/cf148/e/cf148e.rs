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
    let m = scan.token::<usize>();
    let mut f = vec![0usize;m+1];

    for _ in 0..n {
        let k = scan.token::<usize>();
        let mut mx = vec![0;k+1];
        let mut q = Vec::new();
        for _ in 0..k {
            q.push(scan.token::<usize>());
        }
        mx[k] = q.iter().sum();
        for l in 0..k{
            let mut p = 0;
            for r in l..k{
                p += q[r];
                let size = k - (r-l+1);
                mx[size] = mx[size].max(mx[k]-p);
            }
        }
        for j in (1..=m).rev(){
            for (v,&w) in mx.iter().enumerate(){
                if j >= v{
                    f[j] = f[j].max(f[j-v]+w)
                }
            }
        }
    }
    writeln!(out, "{}", f[m]).ok();
}

pub fn run(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    solve(scan,out)
}
// https://codeforces.com/problemset/problem/148/E
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步
