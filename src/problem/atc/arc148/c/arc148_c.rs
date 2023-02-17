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
    #[allow(unused)]
    pub fn token_bytes(&mut self) -> Vec<u8> {
        let s = self.token::<String>();
        return s.as_bytes().into();
    }
}

#[cfg(test)]
mod arc148_c {
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
        b"6 6
1 1 2 2 5
6 1 2 3 4 5 6
3 2 5 6
1 3
3 1 2 3
3 4 5 6
4 2 3 4 5",
        "1
2
1
3
2
3
"
    );
}

// const MOD:usize = 1000000000+7;
// https://atcoder.jp/contests/arc148/tasks/arc148_c
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n:usize = scan.token();

    let q:usize = scan.token();
    // let line = scan.token::<str>();
    // println!("{}",line);
    let mut p = vec![0usize; n + 1];
    let mut child = vec![0usize; n + 1];
    for i in 2..=n {
        p[i] = scan.token();
        child[p[i]] += 1
    }
    for _ in 0..q {
        let mut ans = 0i64;
        let m = scan.token::<usize>();
        let mut s = HashSet::new();
        for _ in 0..m {
            s.insert(scan.token::<usize>());
        }
        for &v in s.iter() {
            ans += child[v] as i64 + 1;
            if s.contains(&p[v]) {
                ans -= 2;
            }
        }
        writeln!(out, "{}", ans).ok();
    }
}
