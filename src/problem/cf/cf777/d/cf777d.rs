use std::cmp::min;
#[allow(unused)]
use std::collections::*;
use std::io::{BufRead, BufWriter, Write};

#[allow(unused)]
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
    pub fn token_bytes(&mut self) -> Vec<u8> {
        let s = self.token::<String>();
        return s.as_bytes().into();
    }
}

// const MOD:usize = 1000000000+7;
// https://codeforces.com/problemset/problem/777/D
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步
pub fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    // 155 ms String切片操作
    let n = scan.token::<usize>();
    let mut a = vec![String::new(); n];
    for i in 0..n {
        a[i] = scan.token::<String>();
    }
    for i in (0..n - 1).rev() {
        if a[i] <= a[i + 1] {
            continue;
        }
        let l = min(a[i].len(), a[i + 1].len());
        if a[i][..l] == a[i + 1][..l] {
            // a[i] = String::from(&a[i][..l]);
            a[i].truncate(l);
            continue;
        }
        for j in 0..l {
            if a[i][j..j + 1] > a[i + 1][j..j + 1] {
                a[i].truncate(j);
                break;
            }
        }
    }
    for s in a {
        writeln!(out, "{}", s).ok();
    }
}
#[allow(unused)]
pub fn solve1(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    // 171 ms Vec[u]操作
    let n = scan.token::<usize>();
    let mut a = vec![vec![]; n];
    for i in 0..n {
        a[i] = scan.token_bytes();
    }
    for i in (0..n - 1).rev() {
        if a[i] <= a[i + 1] {
            continue;
        }
        let l = min(a[i].len(), a[i + 1].len());
        if a[i][..l] == a[i + 1][..l] {
            // a[i] = a[i][..l].to_vec();
            a[i].truncate(l);
            continue;
        }
        for j in 0..l {
            if a[i][j] > a[i + 1][j] {
                a[i].truncate(j);
                break;
            }
        }
    }
    for s in &a {
        writeln!(out, "{}", String::from_utf8_lossy(s).as_ref()).ok();
    }
}
