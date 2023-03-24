#[allow(unused)]
use std::collections::*;
use std::io::{BufRead, BufWriter, Write};

// const MOD:i64 = 1000000000+7;
#[allow(unused)]
pub fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let mut cnt = vec![0usize; n + 1];
    for i in 0..n {
        let v = scan.token::<usize>();
        cnt[v] += 1;
    }
    let mut s = 0;
    let mut mx = HashMap::new();  // 只有最接近上界2的幂的那个数有用
    for &v in cnt.iter() {
        s += v;
        let l = 64 - s.leading_zeros() as usize;
        if s > 0 && s & (s - 1) == 0 {
            mx.insert(l - 1, s);
        } else if !mx.contains_key(&l) || *mx.get(&l).unwrap() < s {
            mx.insert(l, s);
        }
    }
    let mut vs: Vec<&usize> = mx.values().collect();
    vs.sort_unstable();
    fn f(x: usize) -> usize {
        if x == 0 {
            return 1;
        }
        if x & (x - 1) == 0 {
            return 0;
        }
        let l = 64 - x.leading_zeros() as usize;
        let t = 1 << l;
        return t - x;
    }
    let mut ans = f(0) * 2 + f(s);
    let mut suf = 0;
    for y in (0..=n).rev() {
        suf += cnt[y];
        if cnt[y] > 0 {
            ans = ans.min(f(suf) + f(s - suf) + f(0));
            for &&v in vs.iter() {
                if v + suf >= s {
                    break;
                }
                ans = ans.min(f(v) + f(suf) + f(s - v - suf))
            }
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

// https://codeforces.com/contest/1626/problem/D
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步
