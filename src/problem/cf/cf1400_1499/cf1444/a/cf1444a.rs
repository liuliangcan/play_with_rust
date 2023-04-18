#[allow(unused)]
use std::collections::*;
use std::io::{BufRead, BufWriter, Write};

// const MOD:i64 = 1000000000+7;
#[allow(unused)]
pub fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let p = scan.token::<usize>();
    let q = scan.token::<usize>();
    if p % q > 0 {
        writeln!(out, "{p}").ok();
        return;
    }
    let mut ans = 1;
    for (k, v) in get_prime_reasons(q) {
        let mut x = p / k.pow(v as u32);
        while x % k == 0 {
            x /= k;
        }
        // println!("{} {} {}",k,v,x);
        ans = ans.max(x * k.pow(v as u32 - 1))
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

// https://codeforces.com/problemset/problem/1444/A
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步

/// 分解质因数模板
fn get_prime_reasons(mut x: usize) -> HashMap<usize, usize> {
    let mut ans = HashMap::new();
    if x == 1 {
        return ans;
    }
    let mut i = 2;
    while i * i <= x {
        while x % i == 0 {
            *ans.entry(i).or_insert(0) += 1;
            x /= i;
        }
        i += 1;
    }
    if x > 1 {
        *ans.entry(x).or_insert(0) += 1;
    }
    return ans;
}
