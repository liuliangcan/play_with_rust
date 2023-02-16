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

//
pub fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let mut k = scan.token::<usize>();
    let mut a = vec![(0u8, 0i64); 0];
    let mut x = 0i64;
    let mut op = 0u8;
    let mut y = 0i64;
    for _ in 0..n {
        op = scan.token::<u8>();
        y = scan.token::<i64>();
        if op == 1 {
            a.push((op, y - x));
            x = y;
        } else {
            a.push((op, y));
            x += y;
        }
    }
    let mut h = BinaryHeap::new();
    let mut ans = x;
    let mut f = 0i64;

    for &(op, y) in a.iter().rev() {
        if k == 0 {
            break;
        }
        if op == 1 {
            f += y;
            k -= 1;
        } else {
            if y < 0 {
                f += y;
                h.push(y);
            }
        }
        while h.len() > k {
            f -= h.pop().unwrap();
        }
        ans = ans.max(x - f)
    }
    writeln!(out, "{}", ans).ok();
}

// 56ms
pub fn solve1(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let mut k = scan.token::<usize>();
    let mut a = vec![(0u8, 0i64); 0];
    let mut x = 0i64;
    for _ in 0..n {
        let op = scan.token::<u8>();
        let y = scan.token::<i64>();
        if op == 1 {
            a.push((op, y - x));
            x = y;
        } else {
            a.push((op, y));
            x += y;
        }
    }
    let mut h = BinaryHeap::new();
    let mut ans = x;
    let mut f1 = 0i64;
    let mut f2 = 0i64;

    for i in (0..n).rev() {
        if k == 0 {
            break;
        }
        let (op, y) = a[i];
        if op == 1 {
            f1 += y;
            k -= 1;
        } else {
            if y < 0 {
                f2 += y;
                h.push(y);
            }
        }
        while h.len() > k {
            f2 -= h.pop().unwrap();
        }
        ans = ans.max(x - f1 - f2)
    }
    writeln!(out, "{}", ans).ok();
}

pub fn run(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    solve(scan, out)
}
// https://atcoder.jp/contests/abc249/tasks/abc249_f
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步
