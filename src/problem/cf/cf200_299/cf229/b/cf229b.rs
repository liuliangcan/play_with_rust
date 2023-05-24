#[allow(unused)]
use std::collections::*;
use std::io::{BufRead, BufWriter, Write};

// const MOD:i64 = 1000000000+7;
#[allow(unused)]
pub fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let m = scan.token::<usize>();
    let mut g = vec![Vec::new(); n];
    for _ in 0..m {
        let u = scan.token::<usize>() - 1;
        let v = scan.token::<usize>() - 1;
        let w = scan.token::<i64>();
        g[u].push((v, w));
        g[v].push((u, w));
    }
    let mut t = vec![HashSet::new(); n];
    for i in 0..n {
        let mut k = scan.token::<usize>();
        for _ in 0..k {
            t[i].insert(scan.token::<i64>()); // 由于rust是大顶堆，dis要取反
        }
    }
    let mut h = BinaryHeap::new();
    h.push((0, 0));
    let mut dist = vec![10000000000000; n];
    dist[0] = 0;
    while h.len() > 0 {
        let (mut d, u) = h.pop().unwrap();
        d = -d;
        if d > dist[u] {
            continue;
        }
        if u == n - 1 {
            writeln!(out, "{}", d).ok();
            return;
        }
        while t[u].contains(&d) {
            d += 1;
        }
        for &(v, w) in g[u].iter() {
            if d + w < dist[v] {
                dist[v] = d + w;
                h.push((-d - w, v))
            }
        }
    }
    writeln!(out, "-1").ok();
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

// https://codeforces.com/contest/229/problem/B
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步
