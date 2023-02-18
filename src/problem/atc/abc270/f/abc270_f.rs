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
}

// const MOD:i64 = 1000000000+7;
pub fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let m = scan.token::<usize>();
    let (a, p) = (0, n + 1); // 机场和港口的超级源点
    let mut air_edge = vec![(0usize, 0usize, 0usize); 0]; // 记录每个岛屿建机场到超级源点a的花费
    let mut port_edge = vec![(0usize, 0usize, 0usize); 0]; // 港口到超级源点p的花费
    let mut bridge = vec![(0usize, 0usize, 0usize); 0]; // 建桥的花费
    for i in 1..=n {
        let w = scan.token::<usize>();
        air_edge.push((i, a, w))
    }
    for i in 1..=n {
        let w = scan.token::<usize>();
        port_edge.push((i, p, w))
    }
    for _ in 0..m {
        let u = scan.token::<usize>();
        let v = scan.token::<usize>();
        let w = scan.token::<usize>();
        bridge.push((u, v, w))
    }
    fn calc(es: &mut Vec<(usize, usize, usize)>, n: usize) -> usize {
        es.sort_unstable_by_key(|x| x.2);
        let mut dsu = DSU::new(n);
        let mut ans = 0;
        for &mut (u, v, w) in es {
            if dsu.union(u, v) {
                ans += w;
            }
        }
        for i in 1..=n - 2 {
            if dsu.find(i) != dsu.find(1) {
                return 100000000000000000usize;
            }
        }
        return ans;
    }
    let mut ess = vec![bridge.clone(); 4];
    ess[1].extend(air_edge.clone());
    ess[2].extend(port_edge.clone());
    ess[3].extend(air_edge);
    ess[3].extend(port_edge);
    let mut ans = 100000000000000000usize;
    for es in ess.iter_mut() {
        ans = ans.min(calc(es, n + 2));
    }
    writeln!(out, "{}", ans).ok();
}

pub fn run(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    solve(scan, out)
}
// https://atcoder.jp/contests/abc270/tasks/abc270_f
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
            let t = x;
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
