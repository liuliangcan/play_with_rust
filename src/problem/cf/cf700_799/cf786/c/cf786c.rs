#[allow(unused)]
use std::collections::*;
use std::io::{BufRead, BufWriter, Write};

// const MOD:i64 = 1000000000+7;

pub struct Program {
    n: usize,
    clock: usize,
    a: Vec<usize>,
    ans: Vec<usize>,
    time: Vec<usize>,
}
impl Program {
    pub fn new() -> Self {
        Self {
            n: 0,
            clock: 0,
            a: Vec::new(),
            ans: Vec::new(),
            time: Vec::new(),
        }
    }

    fn f(&mut self, k: usize) -> usize {
        let clock = &mut self.clock;
        *clock += 1;
        let mut m = 1;
        let mut cnt = 0;

        let a = &mut self.a;
        let time = &mut self.time;
        for &v in a.iter() {
            if time[v] == *clock {
                continue;
            }
            cnt += 1;
            if cnt > k {
                m += 1;
                *clock += 1;
                cnt = 1;
            }
            time[v] = *clock
        }
        return m;
    }
    fn dfs(&mut self, l: usize, r: usize) {
        if self.ans[l] == 0 {
            self.ans[l] = self.f(l);
        }
        if self.ans[r] == 0 {
            self.ans[r] = self.f(r);
        }
        if l + 1 >= r {
            return;
        }
        if self.ans[l] == self.ans[r] {
            for i in l + 1..r {
                self.ans[i] = self.ans[l]
            }
            return;
        }
        let mid = (l + r) >> 1;
        self.dfs(l, mid);
        self.dfs(mid, r);
    }
    #[allow(unused)]
    pub fn solve(&mut self, scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
        self.n = scan.token::<usize>();
        self.a = Vec::with_capacity(self.n);
        let a = &mut self.a;
        for i in 0..self.n {
            a.push(scan.token::<usize>());
        }
        self.time = vec![0; self.n + 1];
        self.ans = vec![0; self.n + 1];
        self.clock = 0usize;

        self.dfs(1, self.n);
        for i in 1..=self.n {
            write!(out, "{} ", self.ans[i]).ok();
        }
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
