#[allow(unused)]
use std::collections::*;
use std::io::{BufRead, BufWriter, Write};

// const MOD:i64 = 1000000000+7;
// 171 ms 树状数组
#[allow(unused)]
pub fn solve1(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let mut a = vec![0usize; n];
    for i in 0..n {
        a[i] = scan.token::<usize>();
    }
    let mut tree = BIT::new(n * 2 + 1);
    for _ in 0..n {
        let i = scan.token::<usize>() + 1;
        tree.add_point(i, 1);
        tree.add_point(i + n, 1);
    }
    for v in a {
        let p = tree.min_right(n - v + 1) - 1;
        write!(out, "{} ", (v + p) % n).ok();
        tree.add_point(p % n + 1, -1);
        tree.add_point(p % n + n + 1, -1);
    }
}

//  93ms 并查集
#[allow(unused)]
pub fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let mut a = vec![0usize; n];
    for i in 0..n {
        a[i] = scan.token::<usize>();
    }
    let mut cnt = vec![0; 2 * n + 1];
    let mut dsu = DSU::new(n * 2 + 1);
    for _ in 0..n {
        let i = scan.token::<usize>();
        cnt[i] += 1;
        cnt[i + n] += 1;
    }
    for v in a {
        let mut x = n - v;
        while cnt[x] == 0 {
            dsu.union(x, x + 1);
            x = dsu.find(x);
        }
        write!(out, "{} ", (v + x) % n).ok();
        cnt[x % n] -= 1;
        cnt[x % n + n] -= 1;
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

// https://codeforces.com/problemset/problem/1157/E
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
    pub fn sum_prefix(&mut self, mut i: usize) -> i64 {
        let mut s = 0;
        while i >= 1 {
            s += self.c[i];
            i &= i - 1;
        }
        return s;
    }
    /// 计算[l,r]闭区间的和，注意下标1-indexed
    #[allow(unused)]
    pub fn sum_interval(&mut self, l: usize, r: usize) -> i64 {
        return self.sum_prefix(r) - self.sum_prefix(l - 1);
    }

    pub fn low_bit(&self, x: usize) -> usize {
        let x = x as i64;
        return (x & (-x)) as usize;
    }
    /// 查找[i..]后第一个>=0的位置，包含i，注意i是1-indexed
    pub fn min_right(&mut self, i: usize) -> usize {
        let p = self.sum_prefix(i);
        if i == 1 {
            if p > 0 {
                return i;
            }
        } else {
            if p > self.sum_prefix(i - 1) {
                return i;
            }
        }
        let (mut l, mut r) = (i, self.n + 1);
        while l + 1 < r {
            let mid = (l + r) >> 1;
            if self.sum_prefix(mid) > p {
                r = mid
            } else {
                l = mid
            }
        }
        return r;
    }
}

#[allow(unused)]
pub struct DSU {
    n: usize,
    fa: Vec<usize>,
    size: Vec<usize>,
    edge_size: Vec<usize>,
    groups: usize,
}

impl DSU {
    pub fn new(n: usize) -> Self {
        Self {
            n,
            fa: (0..n).collect::<Vec<usize>>(),
            size: vec![1; n],
            edge_size: vec![0; n],
            groups: n,
        }
    }
    pub fn find(&mut self, x: usize) -> usize {
        if self.fa[x] != x {
            self.fa[x] = self.find(self.fa[x]);
        }
        return self.fa[x];
    }
    pub fn union(&mut self, x: usize, y: usize) -> bool {
        let x = self.find(x);
        let y = self.find(y);
        if x == y {
            self.edge_size[x] += 1;
            return false;
        }
        // if self.size[x] == self.size[y] {  // 按轶合并会影响定向合并，常关，老板不支持这个写法，需要用临时变量
        //     (x, y) = (y, x); // 新版可以写成 (x, y) = (y, x)
        // }
        self.fa[x] = y;
        self.size[y] += 1;
        self.edge_size[y] += self.edge_size[x] + 1;
        self.groups -= 1;
        return true;
    }
}
