#[allow(unused)]
use std::collections::*;
// use std::intrinsics::atomic_load_acquire;
use std::io::{BufRead, BufWriter, Write};

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
    pub fn token_bytes(&mut self) -> Vec<u8> {
        let s = self.token::<String>();
        return s.as_bytes().into();
    }
}

#[cfg(test)]
mod abc267_f {
    use super::*;

    macro_rules! test_macro {
        ($name:ident, $input:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let output = &mut Vec::new();
                let scan = &mut Scanner::new($input as &[u8]);
                solve(scan, output);
                assert_eq!($expected, std::str::from_utf8(output).unwrap());
            }
        };
    }

    test_macro!(
        case1,
        b"5
1 2
2 3
3 4
3 5
3
2 2
5 3
3 3",
        "4
1
-1
"
    );
    test_macro!(
        case2,
        b"10
1 2
2 3
3 5
2 8
3 4
4 6
4 9
5 7
9 10
5
1 1
2 2
3 3
4 4
5 5",
        "2
4
10
-1
-1
"
    );
}

// const MOD:usize = 1000000000+7;
// https://atcoder.jp/contests/abc267/tasks/abc267_f
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let mut g = vec![Vec::new(); n];
    for _ in 0..n - 1 {
        let u: usize = scan.token::<usize>() - 1;
        let v: usize = scan.token::<usize>() - 1;
        g[u].push(v);
        g[v].push(u);
    }
    let q: usize = scan.token::<usize>();
    let mut qs = vec![Vec::new(); n];
    for i in 0..q {
        let u: usize = scan.token::<usize>() - 1;
        let k: usize = scan.token::<usize>();
        qs[u].push((i, k))
    }
    let mut ans = vec![-1i32; q];
    // let mut leaf = 0usize;
    // let mut mx = 0;
    let mut s = vec![0usize; 2]; // leaf,mx  由于int形参无法改变，于是包装成数组传进来。
    let mut path = vec![0usize; n];
    fn dfs(
        u: usize,
        fa: usize,
        d: usize,
        g: &Vec<Vec<usize>>,
        ans: &mut Vec<i32>,
        path: &mut Vec<usize>,
        qs: &Vec<Vec<(usize, usize)>>,
        s: &mut Vec<usize>,  // 由于int形参无法改变，于是包装成数组传进来。
    ) {
        path[d] = u;
        if d > s[1] {
            s[1] = d;
            s[0] = u;
        }
        for &(i, k) in &qs[u] {
            if d >= k {
                ans[i] = (path[d - k] + 1) as i32
            }
        }
        for &v in &g[u] {
            if v != fa {
                dfs(v, u, d + 1, g, ans, path, qs, s);
            }
        }
    }
    for _ in 0..3 {
        dfs(s[0], n, 0, &g, &mut ans, &mut path, &qs, &mut s)
    }
    for v in ans {
        writeln!(out, "{}", v).ok();
    }
}
