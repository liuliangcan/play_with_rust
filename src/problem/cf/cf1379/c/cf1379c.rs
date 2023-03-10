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
// 二分46ms
#[allow(unused)]
pub fn solve1(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let m = scan.token::<usize>();
    let mut a = Vec::new();
    let mut ba = Vec::new();
    for _ in 0..m {
        let x = scan.token::<usize>();
        let y = scan.token::<usize>();
        a.push(x);
        ba.push((y, x));
    }
    a.sort_unstable();
    let mut pa = vec![0; m + 1];
    for i in (0..m).rev() {
        pa[i] = pa[i + 1] + a[i];
    }
    pa.reverse();
    let mut ans = 0;
    if n <= m {
        ans = pa[n]
    }
    for &(y, x) in ba.iter() {
        let p = a.partition_point(|v| *v < y);
        let z = m - p;
        if z >= n {
            continue;
        }
        if x >= y {
            ans = ans.max(pa[z] + (n - z) * y)
        } else {
            ans = ans.max(pa[z] + x + (n - z - 1) * y)
        }
    }
    writeln!(out, "{}", ans).ok();
}

#[allow(unused)]
// 双指针64ms
pub fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let mut n = scan.token::<i64>();
    let m = scan.token::<i64>();
    let mut a = Vec::new();
    let mut ba = Vec::new();
    for _ in 0..m {
        let x = scan.token::<i64>();
        let y = scan.token::<i64>();
        a.push(x);
        ba.push((y, x));
    }
    a.sort_unstable();
    ba.sort_unstable();
    let mut i = m - 1;
    let mut s = 0;

    let mut ans = 0;
    for &(y, x) in ba.iter().rev() {
        while n > 0 && i >= 0 && a[i as usize] >= y {
            s += a[i as usize];
            i -= 1;
            n -= 1;
        }
        if n <= 0 {
            ans = ans.max(s);
            break;
        }

        if x >= y {
            ans = ans.max(s + n * y)
        } else {
            ans = ans.max(s + x + (n - 1) * y)
        }
    }
    writeln!(out, "{}", ans).ok();
}

pub fn run(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let t = scan.token::<usize>();
    for _ in 0..t {
        solve(scan, out)
    }
}
// https://codeforces.com/problemset/problem/1379/C
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步
