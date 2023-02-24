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
    let a = scan.token::<i64>();
    let b = scan.token::<i64>();
    let c = scan.token::<i64>();
    let d = scan.token::<i64>();
    fn qiugen(cc: i64) -> [i64; 2] {
        return [
            ((1.0 + ((1 + 8 * cc) as f32).sqrt()) / 2.0) as i64,
            ((1.0 - ((1 + 8 * cc) as f32).sqrt()) / 2.0) as i64,
        ];
    }
    let zeros = qiugen(a as i64);
    let ones = qiugen(d as i64);
    // println!("{:?}", zeros);
    // println!("{:?}", ones);
    for zero in zeros {
        for one in ones {
            if one < 0 || zero < 0 {
                continue;
            }
            if zero * (zero - 1) / 2 != a
                || one * (one - 1) / 2 != d
                || (zero + one) * (zero + one - 1) / 2 != a + b + c + d
            {
                continue;
            }
            if zero == 0 {
                writeln!(out, "{}", "1".repeat(one as usize)).ok();
                return;
            }
            if one == 0 {
                writeln!(out, "{}", "0".repeat(zero as usize)).ok();
                return;
            }
            let back1 = b / zero;
            let m = b % zero;
            let front1 = one - back1;
            let mut zs = vec![b'0'; zero as usize];
            let mut front = vec![b'1'; front1 as usize];
            if m > 0 {
                zs[(m - 1) as usize] = b'1';
                *front.last_mut().unwrap() = b'0';
            }
            write!(out, "{}", String::from_utf8_lossy(&front)).ok();
            write!(out, "{}", String::from_utf8_lossy(&zs)).ok();
            writeln!(out, "{}", "1".repeat(back1 as usize)).ok();
            return;
        }
    }
    writeln!(out, "Impossible").ok();
}

pub fn run(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    solve(scan, out)
}
// https://codeforces.com/problemset/problem/708/B
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步
