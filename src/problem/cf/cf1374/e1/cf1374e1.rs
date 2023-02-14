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

// const MOD:i64 = 1000000000+7;
pub fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let k = scan.token::<usize>();
    let mut a = vec![0i32; 0];
    let mut b = vec![0i32; 0];
    let mut c = vec![0i32; 0];
    for _ in 0..n {
        let t = scan.token::<i32>();
        let x = scan.token::<i32>();
        let y = scan.token::<i32>();
        if x == 1 && y == 1 {
            c.push(t);
        } else if x == 1 {
            a.push(t);
        } else if y == 1 {
            b.push(t);
        }
    }
    if a.len() > b.len() {
        let mut t = a;
        a = b;
        b = t
    }
    if c.len() + a.len() < k {
        writeln!(out, "-1").ok();
        return;
    }
    a.sort_unstable();
    b.sort_unstable();
    c.sort_unstable();
    for i in 0..a.len() {
        a[i] += b[i]
    }
    let both = c.len().min(k);
    let mut ans = c[0..both].iter().sum::<i32>();

    let mut j = k - both;  // a的遍历起点
    ans += a[0..(k - both)].iter().sum::<i32>();

    let mut i = both;  // 为了适应usize不能为负 i右移一位
    while j < a.len() && i > 0 && a[j] < c[i - 1] {
        ans += a[j] - c[i - 1];
        i -= 1;
        j += 1;
    }
    writeln!(out, "{}", ans).ok();
}

pub fn run(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    solve(scan, out)
}
// https://codeforces.com/contest/1374/problem/E1
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步
