#[allow(unused)]
use std::collections::*;
use std::io::{BufRead, BufWriter, Write};

// const MOD:i64 = 1000000000+7;
#[allow(unused)]
pub fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let k = scan.token::<usize>();
    let mut s = scan.token_bytes();
    let mut poses = vec![Vec::new(); b'9' as usize + 1];
    for (i, &c) in s.iter().enumerate() {
        poses[c as usize].push(i);
    }
    let mut mn = 1000000000usize;
    let mut ans = vec![b'1'; 0];
    for i in 0..=9 {
        let c = b'0' + i;
        let mut t = s.clone();
        let mut cost = 0usize;
        let mut remain = k as i32 - poses[c as usize].len() as i32;
        for d in 1..=9 {
            if remain <= 0 || cost > mn {
                break;
            }
            let j = c + d;
            if j <= b'9' {
                let ps = &poses[j as usize];
                let cnt = ps.len().min(remain as usize); // cnt得是usize,但remain可能一开始就是负，根本傻比，需要在外特判先
                for x in 0..cnt {
                    cost += d as usize;
                    t[ps[x]] = c;
                    remain -= 1;
                }
            }
            if remain <= 0 || cost > mn {
                break;
            }
            let j = c - d;
            if j >= b'0' {
                let ps = &poses[j as usize];
                let cnt = ps.len().min(remain as usize);
                // 注意这里，是取ps的最后cnt个数
                // for x in ps.len() - cnt..ps.len() {
                //     cost += d as usize;
                //     t[ps[x]] = c;
                //     remain -= 1;
                // }
                for &x in ps[ps.len() - cnt..].iter(){
                        cost += d as usize;
                        t[x] = c;
                        remain -= 1;
                }
            }
        }
        if remain <= 0 {
            if cost < mn {
                mn = cost;
                ans = t;
            }
            // else if cost == mn  {
            //     for (y,&c) in t.iter().enumerate() {
            //         if c > ans[y]{
            //             break
            //         }else if c < ans[y]{
            //             ans = t.clone();
            //             break
            //         }
            //     }
            // }
            else if cost == mn && ans > t {
                ans = t;
            }
        }
    }
    writeln!(out, "{}", mn).ok();
    writeln!(out, "{}", String::from_utf8_lossy(&ans)).ok();
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

// https://codeforces.com/problemset/problem/118/C
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步
