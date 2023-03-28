#[allow(unused)]
use std::collections::*;
use std::io::{BufRead, BufWriter, Write};

// const MOD:i64 = 1000000000+7;
#[allow(unused)]
pub fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let m = scan.token::<usize>();
    let q = scan.token::<usize>();
    let mut mp = HashSet::new();
    let mut qs = Vec::new();
    let mut ab = vec![vec![0; n.max(m)]; 2];
    for _ in 0..q {
        let t = scan.token::<usize>();
        let i = scan.token::<usize>();
        let y = scan.token::<usize>();
        qs.push((t, i, y));
        mp.insert(y);
    }
    mp.insert(0);
    let mut hs = mp.iter().cloned().collect::<Vec<_>>();
    hs.sort_unstable();
    // println!("{:?}",hs);
    let sz = hs.len();
    let mut trees = vec![
        vec![BIT::new(sz), BIT::new(sz)],
        vec![BIT::new(sz), BIT::new(sz)],
    ];
    trees[0][0].add_point(1, n as i64);
    trees[1][0].add_point(1, m as i64);
    let mut ans = 0;
    for &(t, i, y) in qs.iter() {
        let a = &ab[t - 1];
        let x = a[i - 1];
        ab[t - 1][i - 1] = y;
        // let xx = hs.partition_point(|v| *v < x) + 1;
        // let yy = hs.partition_point(|v| *v < y) + 1;
        let xx = bisect_left(0,hs.len(),|i| hs[i]>=x)+1;
        let yy = bisect_left(0,hs.len(),|i| hs[i]>=y)+1;
        // let mut cnt = &trees[t - 1][0];
        // let mut sum = &trees[t - 1][1];
        trees[t - 1][0].add_point(xx, -1);
        trees[t - 1][0].add_point(yy, 1);
        trees[t - 1][1].add_point(xx, -(x as i64));
        trees[t - 1][1].add_point(yy, y as i64);

        let cnt = &trees[2 - t][0];
        let sum = &trees[2 - t][1];
        ans += cnt.sum_prefix(yy) * y as i64 + sum.sum_interval(yy + 1, sz);
        ans -= cnt.sum_prefix(xx) * x as i64 + sum.sum_interval(xx + 1, sz);
        writeln!(out, "{}",ans).ok();
    }
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

// https://atcoder.jp/contests/jsc2021/tasks/jsc2021_f
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步

pub struct BIT {
    c: Vec<i64>,
    n: usize,
}
impl BIT {
    pub fn new(n: usize) -> Self {
        Self {
            c: vec![0; n + 2],
            n: n,
        }
    }
    /// 给下标i位置+v,注意i是1-indexed
    pub fn add_point(&mut self, mut i: usize, v: i64) {
        while i <= self.n {
            while i <= self.n {
                self.c[i] += v;
                i += self.low_bit(i);
            }
        }
    }
    /// 计算[..i]的前缀和，包含i，注意下标1-indexed
    pub fn sum_prefix(&self, mut i: usize) -> i64 {
        let mut s = 0;
        while i >= 1 {
            s += self.c[i];
            i &= i - 1;
        }
        return s;
    }
    /// 计算[l,r]闭区间的和，注意下标1-indexed
    #[allow(unused)]
    pub fn sum_interval(&self, l: usize, r: usize) -> i64 {
        return self.sum_prefix(r) - self.sum_prefix(l - 1);
    }

    pub fn low_bit(&self, x: usize) -> usize {
        let x = x as i64;
        return (x & (-x)) as usize;
    }
}
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