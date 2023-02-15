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
}


// 77 ms
pub fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let k = scan.token::<usize>();
    let mx = 200000;
    let mut p = vec![vec![0usize; 0]; mx+1];

    for _ in 0..n {
        let v = scan.token::<usize>();
        if p[v].len() == 0{
            p[v].push(1);
        }
        else {
            p[v][0] += 1;
        }
    }

    let mut ans = 111111111;
    for i in (1..=mx).rev() {
        // 由于无法同时处理p[i],p[i2]认为是对p进行了两次引用
        // 因此对引用切片，变成2个引用
        let (x,y) = p.split_at_mut(i);

        let b = &y[0];
        if b.len() == 0{
            continue
        }

        let mut s = 0;
        let mut left = k;
        for j in 0..b.len() {
            if left <= b[j] {
                ans = ans.min(s + left*j);
                break;
            }
            s += j * b[j];
            left -= b[j];
        }
        let i2 = i / 2;
        let mut c = x.get_mut(i2).unwrap();
        if c.len() == 0{
            c.push(0);
            c.extend(b);
        } else {
            for j in 0..b.len() {
                if j + 1 == c.len() {
                    c.extend(b[j..].iter());
                    break;
                }
                c[j + 1] += b[j];
            }
        }
    }
    writeln!(out, "{}", ans).ok();
}

pub fn solve2(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let k = scan.token::<usize>();
    let mx = 200000;
    let mut p = vec![vec![0usize; 0]; mx+1];

    for i in 0..n {
        let v = scan.token::<usize>();
        if p[v].len() == 0{
            p[v].push(1);
        }
        else {
            p[v][0] += 1;
        }
    }

    let mut ans = 111111111;
    for i in (1..=mx).rev() {
        if p[i].len() == 0{
            continue
        }
        let b = p[i].clone();

        let mut s = 0;
        let mut left = k;
        for j in 0..b.len() {
            if left <= b[j] {
                ans = ans.min(s + left*j);
                break;
            }
            s += j * b[j];
            left -= b[j];
        }
        let i2 = i / 2;
        let mut c = &mut p[i2];
        if c.len() == 0{
            c.push(0);
            c.extend(b);
        } else {
            for j in 0..b.len() {
                if j + 1 == c.len() {
                    c.extend(b[j..].iter());
                    break;
                }
                c[j + 1] += b[j];
            }
        }
    }
    writeln!(out, "{}", ans).ok();
}

pub fn solve1(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let k = scan.token::<usize>();
    let mx = 200000;
    let mut a = vec![0i32; n];
    for i in 0..n {
        a[i] = scan.token::<i32>();
    }
    a.sort_unstable();
    let mut p = vec![vec![0i32; 0]; mx+1];
    for mut v in a {
        let mut i = 0 ;
        while v > 0 {
            p[v as usize].push(i);
            i += 1;
            v /= 2;
        }
        p[v as usize].push(i);
    }

    let mut ans = 111111111;
    for x in p {
        if x.len() >= k {
            ans = ans.min(x[0..k].iter().sum::<i32>())
        }
    }

    writeln!(out, "{}", ans).ok();
}

pub fn run(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    solve(scan,out)
}
// https://codeforces.com/problemset/problem/1213/D2
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步
