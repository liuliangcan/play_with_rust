#[allow(unused)]
use std::collections::*;
use std::io::{BufRead, BufWriter, Write};
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

// const MOD:i64 = 1000000000+7;
// sortedlist 280 ms
#[allow(unused)]
pub fn solve1(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let mut a = Vec::new();
    for i in 0..n {
        let l = scan.token::<i64>();
        let r = scan.token::<i64>();
        a.push((r, l, i))
    }
    a.sort_unstable();
    let mut p = SortedList::new();
    let mut ans = vec![0; n];
    for &(_, l, i) in a.iter() {
        ans[i] = p.len() - p.bisect_left(l);
        p.add(l);
    }
    for v in ans {
        writeln!(out, "{}", v).ok();
    }
}
// BIT 156 ms
pub fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let mut a = Vec::new();
    let mut hs = Vec::new();
    for i in 0..n {
        let l = scan.token::<i64>();
        let r = scan.token::<i64>();
        hs.extend([l,r]);
        a.push((r, l, i))
    }
    a.sort_unstable();
    hs.sort_unstable();
    let mut p = BIT::new(n*2);
    let mut ans = vec![0; n];
    for &(_, l, i) in a.iter() {
        let z = hs.partition_point(|x|*x<l);
        ans[i] = p.sum_interval(z+1,n*2+1);
        p.add_point(z+1,1);
    }
    for v in ans {
        writeln!(out, "{}", v).ok();
    }
}

pub fn run(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    solve(scan, out)
}
// https://codeforces.com/problemset/problem/652/D
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步

pub struct SortedList<T> {
    small: Vec<T>,
    large: Vec<T>,
}
impl<T: std::cmp::Ord> SortedList<T> {
    pub fn new() -> Self {
        Self {
            small: Vec::new(),
            large: Vec::new(),
        }
    }
    pub fn add(&mut self, v: T) {
        if self.small.len() > 5000 {
            self.large.append(&mut self.small);
            self.large.sort_unstable();
        }
        // 5000 280 ms
        // 6755 280 ms
        if self.small.len() == 0 || *self.small.last().unwrap() <= v {
            self.small.push(v);
        } else if self.small[0] >= v {
            self.small.insert(0, v);
        } else {
            let p = self.small.partition_point(|x| *x < v);
            self.small.insert(p, v);
        }
    }
    pub fn bisect_left(&mut self, v: T) -> usize {
        return self.small.partition_point(|x| *x < v) + self.large.partition_point(|x| *x < v);
    }
    pub fn len(&self) -> usize {
        return self.small.len() + self.large.len();
    }
}

pub struct BIT{
    c: Vec<i64>,
    n:usize,
}
impl BIT{
    pub fn new(n:usize) -> Self {
        Self {
            c: vec![0;n+2],
            n : n,
        }
    }
    pub fn add_point(&mut self,mut i:usize, v: i64) {
       while i <= self.n{
           while i <= self.n{
               self.c[i] += v;
               i += self.low_bit(i);
           }
       }
    }
    pub fn sum_prefix(&mut self, mut i: usize) -> i64 {
        let mut s = 0;
        while i >= 1{
            s += self.c[i];
            i &= i - 1;
        }
        return s;
    }

    pub fn sum_interval(&mut self,  l: usize,r:usize) -> i64 {
        return self.sum_prefix(r) - self.sum_prefix(l-1);
    }

    pub fn low_bit(&self,x:usize) -> usize {
        let x = x as i64;
        return (x&(-x)) as usize;
    }
}
