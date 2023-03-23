#[allow(unused)]
use std::collections::*;
use std::io::{BufRead, BufWriter, Write};

// const MOD:i64 = 1000000000+7;

#[allow(unused)]
pub fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    // let s = scan.token_bytes();  // 会自动跳过空行所以不需要
    let n = scan.token::<usize>();
    let m = scan.token::<usize>();
    // println!(" {n} {m}");
    let mut two = HashMap::new();
    let mut three = HashMap::new();
    for i in 1..=n {
        let s = scan.token::<String>();
        if m == 1 {
            continue;
        }
        two.insert(String::from(&s[0..2]), (1, 2, i));
        for j in 3..=m {
            two.insert(String::from(&s[j - 2..j]), (j - 1, j, i));
            three.insert(String::from(&s[j - 3..j]), (j - 2, j, i));
        }
    }
    let s = scan.token::<String>();
    if m == 1 {
        writeln!(out, "-1").ok();
        return;
    }
    let mut f = vec![false; m + 1];
    f[0] = true;
    if two.contains_key(&s[..2]) {
        f[2] = true;
    }
    for j in 3..=m {
        if f[j - 2] && two.contains_key(&s[j - 2..j]) {
            f[j] = true
        } else if f[j - 3] && three.contains_key(&s[j - 3..j]) {
            f[j] = true
        }
    }
    if !f[m] {
        writeln!(out, "-1").ok();
        return;
    }
    let mut ans = Vec::new();
    let mut j = m;
    while j > 0 {
        if f[j - 2] && two.contains_key(&s[j - 2..j]) {
            ans.push(two.get(&s[j - 2..j]).unwrap());
            j -= 2
        } else {
            ans.push(three.get(&s[j - 3..j]).unwrap());
            j -= 3
        }
    }
    writeln!(out, "{}", ans.len()).ok();
    for &(l, r, i) in ans.iter().rev() {
        writeln!(out, "{l} {r} {i}").ok();
    }
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

// https://codeforces.com/problemset/problem/1624/E
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步
