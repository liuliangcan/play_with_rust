#[allow(unused)]
use std::collections::*;
use std::io::{BufRead, BufWriter, Write};

// const MOD:i64 = 1000000000+7;

pub struct Program {
    n: usize,
    m: usize,
    w: usize,
    a: Vec<i64>,
    // ans: Vec<usize>,
    // time: Vec<usize>,
}
impl Program {
    pub fn new() -> Self {
        Self {
            n: 0,
            m: 0,
            w: 0,
            a: Vec::new(),
            // ans: Vec::new(),
            // time: Vec::new(),
        }
    }
    pub fn ok(&self, x: usize) -> bool {
        let x = x as i64;
        let mut cnt = 0;
        let mut s = 0;
        let mut d = vec![0i64; self.n + self.w + 1];
        for (i, &v) in self.a.iter().enumerate() {
            s += d[i];
            if s + v < x {
                let delta = x - s - v;
                cnt += delta;
                if cnt as usize > self.m {
                    return true;
                }
                s += delta;
                d[i + self.w] -= delta;
            }
        }
        return false;
    }
    #[allow(unused)]
    pub fn solve(&mut self, scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
        self.n = scan.token::<usize>();
        self.m = scan.token::<usize>();
        self.w = scan.token::<usize>();
        for i in 0..self.n {
            self.a.push(scan.token::<i64>());
        }

        writeln!(
            out,
            "{}",
            bisect_left(
                *self.a.iter().min().unwrap() as usize,
                *self.a.iter().max().unwrap() as usize + self.m + 1,
                |x| self.ok(x)
            ) - 1
        )
        .ok();
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

pub fn bisect_left<P>(lo: usize, hi: usize, mut is_right: P) -> usize
where
    P: FnMut(usize) -> bool,
{
    let mut l = lo;
    let mut r = hi;
    while l < r {
        let mid = l + (r - l) / 2;
        if is_right(mid) {
            r = mid;
        } else {
            l = mid + 1;
        }
    }
    return l;
}
