#[allow(unused)]
use std::collections::*;
use std::io::{BufRead, BufWriter, Write};

fn main() {
    let sin = std::io::stdin();
    let scan = &mut Scanner::new(sin.lock());
    let sout = std::io::stdout();
    let out = &mut BufWriter::new(sout.lock());
    solve(scan, out);
}

#[allow(unused)]
#[macro_export]
macro_rules! logln {
    ($($arg:tt)*) => ({
        #[cfg(debug_assertions)]
        println!($($arg)*);
    })
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
mod abc999x {
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
        test1,
        b"\
3
1 2 1
1 3 3
",
        "\
6
"
    );

    test_macro!(
        test2,
        b"\
5
3 5 2
2 3 2
1 5 1
4 5 13
",
        "\
62
"
    );

    test_macro!(
        test3,
        b"\
10
5 7 459221860242673109
6 8 248001948488076933
3 5 371922579800289138
2 5 773108338386747788
6 10 181747352791505823
1 3 803225386673329326
7 8 139939802736535485
9 10 657980865814127926
2 4 146378247587539124
",
        "\
241240228
"
    );
}

// https://atcoder.jp/contests/abc222/tasks/abc222_a
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let mut adj = vec![Vec::with_capacity(n); n];
    for _ in 0..n - 1 {
        let u = scan.token::<usize>() - 1;
        let v = scan.token::<usize>() - 1;
        let w = scan.token::<usize>();
        adj[u].push((v, w));
        adj[v].push((u, w));
    }
    const MOD: usize = 1000000007;
    let mut ans = 0;
    for i in 0..61 {
        let mut vd = vec![0; 2];
        dfs(&adj, 0, usize::max_value(), 0, &mut vd, i);
        let coe = (vd[0] * vd[1]) % MOD;
        let w = (1 << i) % MOD;
        ans = (ans + coe * w) % MOD;
        logln!("{}", ans);
    }
    writeln!(out, "{}", ans).ok();
    fn dfs(
        adj: &Vec<Vec<(usize, usize)>>,
        u: usize,
        p: usize,
        d: usize,
        vd: &mut Vec<usize>,
        i: usize,
    ) {
        vd[d % 2] += 1;
        for &(v, w) in &adj[u] {
            if p == v {
                continue;
            }
            dfs(adj, v, u, d + (w >> i & 1), vd, i);
        }
    }
}
