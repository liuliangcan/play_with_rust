
#[allow(unused)]
use std::collections::*;
use std::io::{BufRead, BufWriter, Write};
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
    let mut a = vec![0i32; n + 1];
    let mut g = vec![Vec::new(); n + 1];
    let mut d0 = vec![-1; n + 1];
    let mut d1 = vec![-1; n + 1];
    let mut q0 = VecDeque::new();
    let mut q1 = VecDeque::new();
    for i in 1..=n {
        a[i] = scan.token::<i32>();
        if i > a[i] as usize {
            g[i - a[i] as usize].push(i);
        }
        if i + a[i] as usize <= n {
            g[i + a[i] as usize].push(i);
        }
        if a[i] % 2 == 0 {
            q0.push_back(i);
            d0[i] = 0;
        } else {
            q1.push_back(i);
            d1[i] = 0;
        }
    }
    fn bfs(q: &mut VecDeque<usize>, dist: &mut Vec<i32>, g: &Vec<Vec<usize>>) {
        while q.len() > 0 {
            let u = q.pop_front().unwrap();
            for &v in &g[u] {
                if dist[v] == -1 {
                    dist[v] = dist[u] + 1;
                    q.push_back(v);
                }
            }
        }
    }
    bfs(&mut q0, &mut d0, &g);
    bfs(&mut q1, &mut d1, &g);
    for i in 1..=n {
        if a[i] % 2 == 1 {
            write!(out, "{} ", d0[i]).ok();
        } else {
            write!(out, "{} ", d1[i]).ok();
        }
    }
    // 以下在cf上编译不通过：use of undeclared crate or module `itertools`
    // use itertools::Itertools;
    // let ans = (1..=n)
    //     .map(|i| if a[i] % 2 == 1 { d0[i] } else { d1[i] })
    //     .into_iter()
    //     .join(" ");
    // writeln!(out, "{}", ans).ok();
}

pub fn run(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    solve(scan, out)
}
// https://codeforces.com/problemset/problem/1272/E
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步
