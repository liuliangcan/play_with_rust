use itertools::Itertools;
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
// 注意本题点权边权<=1e9，累加2e5次，因此需要用i64
pub fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let mut g = vec![Vec::new(); n];
    let mut d = vec![0i64; n];
    for _ in 1..n {
        let u = scan.token::<usize>() - 1;
        let v = scan.token::<usize>() - 1;
        let w = scan.token::<i64>();
        g[u].push((v, w));
        g[v].push((u, w));
    }
    for i in 0..n {
        d[i] = scan.token::<i64>();
    }
    let mut down1 = vec![0i64; n];
    let mut down2 = vec![0i64; n];
    let mut up = vec![0i64; n];
    fn dfs(
        u: usize,
        fa: usize,
        g: &Vec<Vec<(usize, i64)>>,
        down1: &mut Vec<i64>,
        down2: &mut Vec<i64>,
        d: &Vec<i64>,
    ) {
        for &(v, w) in &g[u] {
            if v == fa {
                continue;
            }
            dfs(v, u, g, down1, down2, d);
            let s = down1[v].max(d[v]) + w;
            if s > down1[u] {
                down2[u] = down1[u];
                down1[u] = s;
            } else if s > down2[u] {
                down2[u] = s;
            }
        }
    }
    dfs(0, n, &g, &mut down1, &mut down2, &d);
    fn reroot(
        u: usize,
        fa: usize,
        g: &Vec<Vec<(usize, i64)>>,
        down1: &mut Vec<i64>,
        down2: &mut Vec<i64>,
        d: &Vec<i64>,
        up: &mut Vec<i64>,
    ) {
        for &(v, w) in &g[u] {
            if v == fa {
                continue;
            }
            if down1[u] == down1[v].max(d[v]) + w {
                up[v] = up[u].max(d[u]).max(down2[u]) + w;
            } else {
                up[v] = up[u].max(d[u]).max(down1[u]) + w;
            }
            reroot(v, u, g, down1, down2, d, up);
        }
    }
    reroot(0, n, &g, &mut down1, &mut down2, &d, &mut up);
    writeln!(out, "{}", (0..n).map(|i| down1[i].max(up[i])).join("\n")).ok();
}

pub fn run(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    solve(scan, out)
}
// https://atcoder.jp/contests/abc222/tasks/abc222_f
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步
