use std::cmp::min;
#[allow(unused)]
use std::collections::*;
use std::io::{BufRead, BufWriter, Write};

fn main() {
    let sin = std::io::stdin();
    let scan = &mut Scanner::new(sin.lock());
    let sout = std::io::stdout();
    let out = &mut BufWriter::new(sout.lock());
    solve(scan, out);
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
    //    pub fn token_bytes(&mut self) -> Vec<u8> {
    //        let s = self.token::<String>();
    //        return s.as_bytes().into();
    //    }
}

// const MOD:usize = 1000000000+7;
// https://atcoder.jp/contests/arc111/tasks/arc111_b
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步
pub fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let mut g = vec![Vec::new(); 400001];
    for i in 0..n {
        let u = scan.token::<usize>();
        let v = scan.token::<usize>();
        g[u].push(v);
        g[v].push(u);
    }
    // bfs方法计算每个连通块的点数和边数，取小的那个(无环树取边数；有环取点数)
    // 并查集方法用py实现；dfs方法用go实现；bfs方法用rust实现；可移步查看
    let mut ans = 0;
    let mut vis = vec![false; g.len()];
    let mut q = VecDeque::new();
    for u in 0..vis.len() {
        if !vis[u] && g[u].len() > 0 {
            let mut cnt_u = 0;
            let mut cnt_e = 0;
            q.push_back(u);
            vis[u] = true;
            while q.len() > 0 {
                let u = q.pop_front().unwrap();
                cnt_u += 1;
                cnt_e += g[u].len();
                for &v in &g[u] {
                    if !vis[v] {
                        vis[v] = true;
                        q.push_back(v);
                    }
                }
            }
            ans += min(cnt_u, cnt_e / 2)
        }
    }
    writeln!(out, "{}", ans).ok();
}
