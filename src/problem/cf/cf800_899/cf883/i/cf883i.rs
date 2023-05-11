#[allow(unused)]
use std::collections::*;
use std::io::{BufRead, BufWriter, Write};

// const MOD:i64 = 1000000000+7;
#[allow(unused)]
pub fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let k = scan.token::<usize>();
    let mut a = Vec::with_capacity(n);
    for i in 0..n {
        a.push(scan.token::<usize>());
    }
    a.sort_unstable();
    fn ok(x: usize, k: usize, n: usize, a: &Vec<usize>) -> bool {
        let mut f = vec![false; n + 1];
        f[0] = true;
        let mut j = 0;
        for i in k - 1..n {
            while a[i] - a[j] > x {
                j += 1;
            }
            while j + k <= i + 1 {
                if f[j] {
                    f[i + 1] = true;
                    break;
                }
                j += 1;
            }
        }
        return f[n];
    }

    writeln!(
        out,
        "{}",
        bisect_left(0, a[n - 1] - a[0], |x| ok(x, k, n, &a))
    )
    .ok();
}

pub fn run(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    solve(scan, out)
}

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

// https://codeforces.com/problemset/problem/883/I
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步
pub fn bisect_left<P>(lo: usize, hi: usize, mut is_right: P) -> usize
where
    P: FnMut(usize) -> bool,
{
    let mut l = lo;
    let mut r = hi;
    while l < r {
        let mid = l + (r - l) / 2;
        if is_right(mid) {
            r = mid;
        } else {
            l = mid + 1;
        }
    }
    return l;
}
