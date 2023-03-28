#[allow(unused)]
use std::collections::*;
use std::io::{BufRead, BufWriter, Write};

// const MOD:i64 = 1000000000+7;
#[allow(unused)]
pub fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let q = scan.token::<usize>();
    let s = "1231";

    let mut a = vec![(0, -1); 1]; // 记录所有节点 (父节点下标，当前节点值)
    let mut cur = 0; // 初始化无节点在0
    let mut b = HashMap::new();
    for _ in 0..q {
        let op = scan.token_bytes();
        if op[0] == b'A' {
            let x = scan.token::<i64>();
            a.push((cur, x)); // 新节点的父节点是当前节点
            cur = a.len() - 1;
        } else if op[0] == b'D' {
            cur = a[cur].0
        } else if op[0] == b'S' {
            let y = scan.token::<i64>();
            b.insert(y, cur);
        } else {
            let y = scan.token::<i64>();
            cur = *b.get(&y).unwrap_or(&0);
        }
        write!(out, "{} ", a[cur].1).ok();
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

// https://atcoder.jp/contests/abc273/tasks/abc273_e
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步
