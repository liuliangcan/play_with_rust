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
pub fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let k = scan.token::<usize>();
    let x = scan.token::<usize>();
    let n = scan.token::<usize>();
    let m = scan.token::<usize>();
    for a1 in 0..=1 {
        for c1 in 0..=1 {
            if a1 + c1 > n {
                continue;
            }
            for ac1 in 0..=(n - a1 - c1) / 2 {
                for a2 in 0..=1 {
                    for c2 in 0..=1 {
                        if a2 + c2 > m {
                            continue;
                        }
                        for ac2 in 0..=(m - a2 - c2) / 2 {
                            let (mut _a1, mut _ac1, mut _c1, mut _a2, mut _ac2, mut _c2) =
                                (a1, ac1, c1, a2, ac2, c2);
                            for _ in 2..k {
                                (_c1, _ac1, _a1, _c2, _ac2, _a2) =
                                    (_c2, _ac2, _a2, _c1, _ac1 + _ac2 + (_a1 & _c2), _a2);
                                if _ac2 > x {
                                    break;
                                }
                            }
                            if _ac2 == x {
                                writeln!(
                                    out,
                                    "{}{}{}{}",
                                    "C".repeat(c1),
                                    "AC".repeat(ac1),
                                    "Z".repeat(n - c1 - a1 - ac1 * 2),
                                    "A".repeat(a1)
                                )
                                .ok();
                                writeln!(
                                    out,
                                    "{}{}{}{}",
                                    "C".repeat(c2),
                                    "AC".repeat(ac2),
                                    "Z".repeat(m - c2 - a2 - ac2 * 2),
                                    "A".repeat(a2)
                                )
                                .ok();
                                return;
                            }
                        }
                    }
                }
            }
        }
    }
    writeln!(out, "Happy new year!").ok();
}

pub fn run(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    // let t = scan.token::<usize>();
    // for _ in 0..t {
    //     solve(scan, out)
    // }
    solve(scan, out)
}
// https://codeforces.com/problemset/problem/379/D
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步
