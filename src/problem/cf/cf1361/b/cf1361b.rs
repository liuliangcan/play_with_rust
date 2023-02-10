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
//    pub fn token_bytes(&mut self) -> Vec<u8> {
//        let s = self.token::<String>();
//        return s.as_bytes().into();
//    }
}


fn pow_mod(mut a: i64, mut b: i64, p: i64) -> i64 {
    let mut ans = 1;
    while b > 0 {
        if b % 2 == 1 {
            ans = ans * a % p
        }
        a = a * a % p;
        b /= 2;
    }
    return ans;
}

const MOD:i64 = 1000000000+7;
pub fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let p = scan.token::<i64>();
    let mut a = vec![0i32; n];
    for i in 0..n {
        a[i] = scan.token::<i32>();
    }
    if p == 1 {
        writeln!(out, "{}", n % 2).ok();
        return;
    }
    a.sort();
    let mut st = vec![(0, 0); 0];
    let mut target_k = a[n - 1];
    let mut i:i32 = n as i32 - 2;
    while i >= 0 {
        let mut k = a[i as usize];
        while st.len() > 0 && st[st.len() - 1] == (k, p - 1) {
            st.pop();
            k += 1;
        }
        if k == target_k {
            if i == 0 {
                writeln!(out, "0").ok();
                return;
            }
            i -= 1;
            target_k = a[i as usize];
        } else if st.len() > 0 && st[st.len() - 1].0 == k {
            let back = st.len() - 1;
            st[back].1 += 1;
        } else {
            st.push((k, 1));
        }
        i -= 1;
    }

    let mut ans = pow_mod(p, target_k as i64, MOD as i64);
    for &(k, c) in &st {
        ans -= pow_mod(p, k as i64, MOD as i64) * c;
    }
    writeln!(out, "{}", (ans % MOD + MOD) % MOD).ok();
}

pub fn run(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let t = scan.token::<usize>();
    for _ in 0..t {
        solve(scan, out)
    }
    // solve(scan,out)
}
// https://codeforces.com/problemset/problem/1361/B
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步
