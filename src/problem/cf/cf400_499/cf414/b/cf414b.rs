#[allow(unused)]
use std::collections::*;
use std::io::{BufRead, BufWriter, Write};

const MOD:usize = 1000000000+7;

pub struct Program {
    // n: usize,
    // clock: usize,
    // a: Vec<usize>,
    // ans: Vec<usize>,
    // time: Vec<usize>,
}
impl Program {
    pub fn new() -> Self {
        Self {
            // n: 0,
            // clock: 0,
            // a: Vec::new(),
            // ans: Vec::new(),
            // time: Vec::new(),
        }
    }
    #[allow(unused)]
    pub fn solve(&mut self, scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
        let mut u = scan.token::<usize>();
        let mut n = scan.token::<usize>();
        let mut f = vec![1;u];
        for i in 1..n {
            let mut g = vec![0;u];
            for (j,&v) in f.iter().enumerate(){
                let mut p = j+1;
                while p <= u{
                    g[p-1] = (g[p-1]+v)%MOD;
                    p += j+1;
                }
            }
            f = g;
        }


        writeln!(out,"{}",f.iter().sum::<usize>()%MOD).ok();
    }
}
pub fn run(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let mut p = Program::new();

    p.solve(scan, out)
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
// https://codeforces.com/contest/786/problem/C
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步
