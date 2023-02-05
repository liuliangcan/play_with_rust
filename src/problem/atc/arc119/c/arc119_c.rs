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
mod arc119_c {
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
5 8 8 6 6",
        "3
"
    );
    test_macro!(
        case2,
        b"7
12 8 11 3 3 13 2",
        "3
"
    );
    test_macro!(
        case3,
        b"10
8 6 3 9 5 4 7 2 1 10",
        "1
"
    );
    test_macro!(case4,
        b"14
630551244 683685976 249199599 863395255 667330388 617766025 564631293 614195656 944865979 277535591 390222868 527065404 136842536 971731491",
        "8
"
    );
}

// const MOD:usize = 1000000000+7;
// https://atcoder.jp/contests/arc119/tasks/arc119_c
pub fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let mut cnt = HashMap::new();
    let mut s = 0i64;
    let mut ans = 0i64;
    cnt.insert(0, 1i64);
    for i in 0..n {
        let v = scan.token::<i64>();
        s += v * (i as i64 % 2 * 2 - 1);
        // if let Some(&c) = cnt.get(&s) {
        //     ans += c;
        // }
        // cnt.entry(s).and_modify(|c| *c += 1).or_insert(1);
        ans += cnt.get(&s).unwrap_or(&0);
        *cnt.entry(s).or_insert(0) += 1;
    }

    writeln!(out, "{}", ans).ok();
}
