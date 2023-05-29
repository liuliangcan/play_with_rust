#[allow(unused)]
use std::collections::*;
use std::io::{BufRead, BufWriter, Write};

// const MOD:i64 = 1000000000+7;

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
        let n = scan.token::<usize>();
        let k = n / 3;
        let mut s = scan.token_bytes();
        let mut cnt = vec![0; 3];
        for &c in s.iter() {
            cnt[(c - '0' as u8) as usize] += 1;
        }
        let mut ans = s.clone();
        for (i, &c) in s.iter().enumerate() {
            let c = (c - '0' as u8) as usize;
            if cnt[c] > k {
                for j in 0..c {
                    if cnt[j] < k {
                        cnt[j] += 1;
                        cnt[c] -= 1;
                        ans[i] = (j + '0' as usize) as u8;
                        break;
                    }
                }
            }
        }
        for (i, &c) in s.iter().enumerate().rev() {
            let c = (c - '0' as u8) as usize;
            if cnt[c] > k {
                for j in (c + 1..=2).rev() {
                    if cnt[j] < k {
                        cnt[j] += 1;
                        cnt[c] -= 1;
                        ans[i] = (j + '0' as usize) as u8;
                        break;
                    }
                }
            }
        }

        writeln!(out, "{}", String::from_utf8_lossy(&ans)).ok();
    }
}
pub fn run(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let mut p = Program::new();

    // for _ in 0..scan.token::<usize>(){
    //     p.solve(scan, out)
    // }
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
