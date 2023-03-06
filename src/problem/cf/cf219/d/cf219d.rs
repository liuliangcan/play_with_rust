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
    let mut g = vec![Vec::new(); n];

    for _ in 0..n-1 {
        let u = scan.token::<usize>() - 1;
        let v = scan.token::<usize>() - 1;
        g[u].push((v, 1));
        g[v].push((u, -1))
    }
    let mut f = vec![0; n]; // 以i为根时，反边数量
    fn dfs(u: usize, fa: usize, g: &Vec<Vec<(usize, i32)>>, f: &mut Vec<i32>) {
        for &(v, d) in g[u].iter() {
            if v != fa {
                dfs(v, u, g, f);
                if d < 0 {
                    f[0] += 1;
                }
            }
        }
    }
    fn reroot(u: usize, fa: usize, g: &Vec<Vec<(usize, i32)>>, f: &mut Vec<i32>) {
        for &(v, d) in g[u].iter() {
            if v != fa {
                f[v] = f[u] + d;
                reroot(v, u, g, f);
            }
        }
    }
    dfs(0, n, &g, &mut f);
    reroot(0, n, &g, &mut f);
    let mn = *f.iter().min().unwrap();
    let ans = (0..n).filter(|i|f[*i] ==mn).collect::<Vec<_>>();

    writeln!(out, "{}", mn).ok();
    for v in ans {
        write!(out, "{} ", v + 1).ok();
    }
}

pub fn run(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    solve(scan, out)
}
// https://codeforces.com/problemset/problem/219/D
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步
