#[allow(unused)]
use std::collections::*;
use std::io::{BufRead, BufWriter, Write};
#[allow(unused)]
fn main() {
    let sin = std::io::stdin();
    let scan = &mut Scanner::new(sin.lock());
    let sout = std::io::stdout();
    let out = &mut BufWriter::new(sout.lock());
    solve(scan, out);
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

// const MOD:usize = 1000000000+7;
// https://codeforces.com/problemset/problem/1141/F2
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步
#[allow(unused)]
pub fn solve1(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let mut a = vec![0i32; n + 1];
    let mut ans = HashMap::<i32, Vec<(usize, usize)>>::new(); // 直接储存贪心结果，在读入时同时处理
    let mut mx = 1000000000i32;
    for r in 1..=n {
        a[r] = scan.token::<i32>();
        let mut s = 0;
        for l in (1..=r).rev() {
            s += a[l];
            if mx == 1000000000i32 {
                mx = s;
            }
            if !ans.contains_key(&s) {
                ans.insert(s, vec![(l, r); 1]);
            } else {
                let ps = ans.get_mut(&s).unwrap();
                let size = ps.len();
                if l > ps[size - 1].1 {
                    ps.push((l, r));
                    if size + 1 > ans.get(&mx).unwrap().len() {
                        mx = s
                    }
                }
            }
        }
    }
    let p = ans.get(&mx).unwrap();
    writeln!(out, "{}", p.len()).ok();
    for &(l, r) in p {
        writeln!(out, "{l} {r}").ok();
    }
}
pub fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let mut a = vec![0i32; n + 1];
    let mut ans = HashMap::<i32, Vec<(usize, usize)>>::new(); // 直接储存贪心结果，在读入时同时处理
    let mut mx = 1000000000i32;
    for r in 1..=n {
        a[r] = scan.token::<i32>();
        let mut s = 0;
        for l in (1..=r).rev() {
            s += a[l];
            if mx == 1000000000i32 {
                mx = s;
            }
            if !ans.contains_key(&s) {
                ans.insert(s, vec![(l, r); 1]);
            } else {
                let ps = ans.get_mut(&s).unwrap();
                let size = ps.len();
                if l > ps[size - 1].1 {
                    ps.push((l, r));
                    if size + 1 > ans.get(&mx).unwrap().len() {
                        mx = s
                    }
                }
            }
        }
    }
    let p = ans.get(&mx).unwrap();
    writeln!(out, "{}", p.len()).ok();
    p.into_iter()
        .for_each(|(l, r)| writeln!(out, "{l} {r}").ok().unwrap()); // 358ms
}
