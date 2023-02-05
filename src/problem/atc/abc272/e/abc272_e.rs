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
mod abc272_e {
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
        b"3 3
-1 -1 -6",
        "2
2
0
"
    );
    test_macro!(
        case2,
        b"5 6
-2 -2 -5 -7 -15",
        "1
3
2
0
0
0
"
    );
    test_macro!(
        case3,
        b"79 6
24 -18 38 -116 -142 -121 -241 -24 -1 9 -63 56 -929 -447 -1077 28 -938 -1249 -846 -166 -1227 -1052 -1376 -602 -1547 -1380 -147 -1360 -2271 -1273 -1143 -2399 -617 -1912 -453 -1254 -721 -882 -2597 -2040 -2968 -1552 -1614 -260 -3387 -2344 -1043 -474 -940 -3595 -2682 -1692 -1119 -670 -3622 -938 -1122 -3082 -3548 -2990 -3683 -2598 -4095 -4146 -3451 -238 -1122 -4435 -2035 -3583 -2891 58 -3662 -1497 -1800 -4594 -2721 -326 -5476",
        "0
0
1
0
0
0
"
    );
}

// const MOD:usize = 1000000000+7;
// https://atcoder.jp/contests/abc272/tasks/abc272_e
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let m = scan.token::<usize>();
    let mut b = vec![vec![]; m + 1];

    for i in 1..=n {
        let v = scan.token::<i64>();
        let mut l = 1;
        if v < 0 {
            l = ((-v) as usize + i - 1) / i;
        }
        for j in l..=m {
            let p = (v + (i * j) as i64) as usize;
            if p < n {
                b[j].push(p)
            } else {
                break;
            }
        }
    }

    let mut t = vec![0usize; n + 1];
    for j in 1..=m {
        for &v in &(b[j]) {
            t[v] = j;
        }
        let mut mex = 0;
        while t[mex] == j {
            mex += 1
        }
        writeln!(out, "{}", mex).ok();
    }
}
