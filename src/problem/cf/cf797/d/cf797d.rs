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
    let mut a = Vec::new();
    let mut cnt = HashMap::new();
    let mut has_fa = vec![false; n];
    for _ in 0..n {
        let v = scan.token::<i32>();
        let l = scan.token::<i32>() - 1;
        let r = scan.token::<i32>() - 1;
        *cnt.entry(v).or_insert(0) += 1;
        if l >= 0 {
            has_fa[l as usize] = true;
        }
        if r >= 0 {
            has_fa[r as usize] = true;
        }
        a.push((v, l, r));
    }
    let mut root = 0;
    while has_fa[root] {
        root += 1;
    }
    let mut ans = n;
    let mut q = VecDeque::new();
    q.push_back((root, 0, 1000000000));
    while !q.is_empty() {
        let (i, l, r) = q.pop_front().unwrap();
        let (v, x, y) = a[i];
        if l <= v && v <= r {
            ans -= cnt.get(&v).unwrap();
        }
        if l < v && x >= 0 {
            q.push_back((x as usize, l, r.min(v - 1)))
        }
        if r > v && y >= 0 {
            q.push_back((y as usize, l.max(v + 1), r))
        }
    }
    writeln!(out, "{}", ans).ok();
}

pub fn run(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    solve(scan, out)
}
// https://codeforces.com/contest/797/problem/D
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步
