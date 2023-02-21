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

pub fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let x = scan.token::<usize>();
    let mut a = vec![0usize; n * 2];
    for i in 0..n {
        a[i] = scan.token::<usize>();
        a[i + n] = a[i]
    }
    let mut q: VecDeque<(usize, usize)> = VecDeque::new(); // 队列模拟滑窗
    let mut ans = 0;
    let mut p = 0; // 滑窗里的天数
    let mut s = 0; // 滑窗里的钱数
    for &v in a.iter() {
        q.push_back((1, v)); // 由于最优方案一定使滑窗末尾在某月末尾，因此直接滑窗纳入本月
        s += (v + 1) * v / 2;
        p += v;
        while p > x {
            // 滑窗里天数多了，从左边月尝试移除多余的天数
            let (l, r) = q.pop_front().unwrap();
            let diff = p - x;
            if r - l + 1 <= diff {
                p -= r - l + 1;
                s -= (l + r) * (r - l + 1) / 2;
            } else {
                s -= (l + l + diff - 1) * diff / 2;
                p = x;
                q.push_front((l + diff, r))
            }
        }
        ans = ans.max(s);
    }
    writeln!(out, "{}", ans).ok();
}

pub fn run(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    solve(scan, out)
}
// https://codeforces.com/problemset/problem/1358/D
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步
