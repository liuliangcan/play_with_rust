#[allow(unused)]
use std::collections::*;
use std::io::{BufRead, BufWriter, Write};

#[allow(unused)]
// use petgraph::unionfind::UnionFind;
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
}

pub fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let mut degree = vec![0i32; n];
    let mut g = vec![Vec::new(); n];
    for _ in 0..n {
        let u = scan.token::<usize>() - 1;
        let v = scan.token::<usize>() - 1;
        g[u].push(v);
        g[v].push(u);
        degree[u] += 1;
        degree[v] += 1;
    }
    let mut q = VecDeque::new();
    for i in 0..n {
        if degree[i] == 1 {
            q.push_back(i);
        }
    }
    let mut dsu = DSU::new(n);
    while q.len() > 0 {
        let u = q.pop_front().unwrap();
        for &v in &g[u] {
            dsu.union(u, v);
            degree[v] -= 1;
            if degree[v] == 1 {
                q.push_back(v);
            }
        }
    }
    let k = scan.token::<usize>();
    for _ in 0..k {
        let u = scan.token::<usize>() - 1;
        let v = scan.token::<usize>() - 1;
        if dsu.find(u) == dsu.find(v) {
            writeln!(out, "Yes").ok();
        } else {
            writeln!(out, "No").ok();
        }
    }
}

pub fn run(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    solve(scan, out)
}
// https://atcoder.jp/contests/abc266/tasks/abc266_f
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步

#[allow(unused)]
pub struct DSU {
    n: usize,
    fa: Vec<usize>,
    size: Vec<usize>,
    edge_size: Vec<usize>,
    groups: usize,
}

impl DSU {
    pub fn new(n: usize) -> Self {
        Self {
            n,
            fa: (0..n).collect::<Vec<usize>>(),
            size: vec![1; n],
            edge_size: vec![0; n],
            groups: n,
        }
    }
    pub fn find(&mut self, x: usize) -> usize {
        if self.fa[x] != x {
            self.fa[x] = self.find(self.fa[x]);
        }
        return self.fa[x];
    }
    pub fn union(&mut self, x: usize, y: usize) -> bool {
        let mut x = self.find(x);
        let mut y = self.find(y);
        if x == y {
            self.edge_size[x] += 1;
            return false;
        }
        if self.size[x] == self.size[y] {
            // (x, y) = (y, x); // cf以外可以写成 (x, y) = (y, x)
            let t= x;
            x = y;
            y = t;
        }
        self.fa[x] = y;
        self.size[y] += 1;
        self.edge_size[y] += self.edge_size[x] + 1;
        self.groups -= 1;
        return true;
    }
}
