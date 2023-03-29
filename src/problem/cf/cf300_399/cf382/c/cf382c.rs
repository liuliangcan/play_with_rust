#[allow(unused)]
use std::collections::*;
use std::io::{BufRead, BufWriter, Write};

// const MOD:i64 = 1000000000+7;
#[allow(unused)]
pub fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let mut a = vec![0; n];
    for i in 0..n {
        a[i] = scan.token::<i64>();
    }
    if n == 1 {
        writeln!(out, "-1").ok();
        return;
    }
    a.sort_unstable();
    if a[0] == a[a.len() - 1] {
        writeln!(out, "1\n{}", a[0]).ok();
        return;
    }
    let mut ans = Vec::new();
    if n == 2 {
        let d = a[1] - a[0];
        ans.push(a[0] - d);
        if d & 1 == 0 {
            ans.push(a[0] + d / 2);
        }
        ans.push(a[1] + d);
        writeln!(out, "{}", ans.len()).ok();
        for v in ans.iter() {
            write!(out, "{v} ").ok();
        }
        return;
    }
    let mut cnt = HashMap::new();
    for i in 0..n - 1 {
        *cnt.entry(a[i + 1] - a[i]).or_insert(0) += 1;
        if cnt.len() >= 3 {
            writeln!(out, "0").ok();
            return;
        }
    }
    if cnt.len() == 1 {
        let d = cnt.keys().max().unwrap();
        writeln!(out, "2\n{} {}", a[0] - d, a[a.len() - 1] + d).ok();
        return;
    }
    if cnt.len() == 2 {
        // let mut c = cnt.iter().collect_vec();
        let mut c = Vec::new();
        for (k,v) in cnt.iter() {
            c.push((k,v));
        }
        c.sort_unstable();
        if *c[1].1 != 1 || *c[1].0 != *c[0].0 * 2 {
            writeln!(out, "0").ok();
            return;
        }
        writeln!(out, "1");
        for i in 0..n - 1 {
            if a[i + 1] - a[i] == *c[1].0 {
                writeln!(out, "{}", a[i] + c[0].0);
                return;
            }
        }
    }
    writeln!(out, "0").ok();
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

// https://codeforces.com/problemset/problem/382/C
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步
