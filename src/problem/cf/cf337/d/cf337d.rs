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
    let m = scan.token::<usize>();
    let d = scan.token::<i32>();

    let mut up: Vec<i32> = vec![i32::MIN; n];
    let mut down1: Vec<i32> = vec![i32::MIN; n];
    let mut down2: Vec<i32> = vec![i32::MIN; n];
    for _ in 0..m {
        down1[scan.token::<usize>() - 1] = 0;
    }
    let mut g: Vec<Vec<usize>> = vec![Vec::new(); n];
    for _ in 0..n - 1 {
        let u = scan.token::<usize>() - 1;
        let v = scan.token::<usize>() - 1;
        g[u].push(v);
        g[v].push(u);
    }
    fn dfs(u: usize, fa: usize, g: &Vec<Vec<usize>>, down1: &mut Vec<i32>, down2: &mut Vec<i32>) {
        for &v in g[u].iter() {
            if v == fa {
                continue;
            }
            dfs(v, u, g, down1, down2);
            let c = down1[v] + 1;
            if c > down1[u] {
                down2[u] = down1[u];
                down1[u] = c;
            } else if c > down2[u] {
                down2[u] = c;
            }
        }
    }
    fn reroot(
        u: usize,
        fa: usize,
        g: &Vec<Vec<usize>>,
        down1: &mut Vec<i32>,
        down2: &mut Vec<i32>,
        up: &mut Vec<i32>,
    ) {
        for &v in g[u].iter() {
            if v == fa {
                continue;
            }
            if down1[v] + 1 == down1[u] {
                up[v] = up[u].max(down2[u]) + 1;
            } else {
                up[v] = up[u].max(down1[u]) + 1
            }
            reroot(v, u, g, down1, down2, up)
        }
    }
    dfs(0, n, &g, &mut down1, &mut down2);
    reroot(0, n, &g, &mut down1, &mut down2, &mut up);
    let ans = (0..n).filter(|&i| up[i].max(down1[i]) <= d).count();
    writeln!(out, "{}", ans).ok();
}

pub fn run(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    solve(scan, out)
}
// https://codeforces.com/problemset/problem/337/D
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步
