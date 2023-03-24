#[allow(unused)]
use std::collections::*;
use std::io::{BufRead, BufWriter, Write};

// const MOD:i64 = 1000000000+7;
#[allow(unused)]
// 62 ms
pub fn solve1(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let k = scan.token::<usize>();
    let mut a = vec![0; n];
    for i in 0..n {
        a[i] = scan.token::<usize>();
    }
    fn f(x: usize, mut i: usize, n: usize, a: &Vec<usize>, k: usize) -> bool {
        let mut cnt = i;
        while i < n {
            if a[i] <= x {
                cnt += 1;
                i += 1;
                if i < n {
                    cnt += 1;
                    i += 1;
                }
                continue;
            }
            i += 1;
        }
        return cnt >= k;
    }

    // let z: Vec<usize> = (0..()).collect();
    let mut l = -1;
    let mut r = *a.iter().max().unwrap() as i32 + 1;
    while l + 1 < r {
        let mid = ((l + r) / 2) as usize;
        if f(mid, 0, n, &a, k) || f(mid, 1, n, &a, k) {
            r = mid as i32;
        } else {
            l = mid as i32;
        }
    }

    let mut ans = r;
    writeln!(out, "{}", ans).ok();
}

pub fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let k = scan.token::<usize>();
    let mut a = vec![0; n];
    for i in 0..n {
        a[i] = scan.token::<usize>();
    }
    fn f(x: usize, mut i: usize, n: usize, a: &Vec<usize>, k: usize) -> bool {
        let mut cnt = i;
        while i < n {
            if a[i] <= x {
                cnt += 1;
                i += 1;
                if i < n {
                    cnt += 1;
                    i += 1;
                }
                continue;
            }
            i += 1;
        }
        return cnt >= k;
    }


    let ans = bisect_left(0, 1e9 as usize, |x| f(x, 0, n, &a, k) || f(x, 1, n, &a, k));
    writeln!(out, "{}", ans).ok();
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

// https://codeforces.com/problemset/problem/1370/D
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步

/// 二分模板 采用左闭右开写法，这样就可以都usize
/// 传入下界上界，以及函数判断这个值是否是在右半部分
/// 返回右半部分的第一个下标
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
