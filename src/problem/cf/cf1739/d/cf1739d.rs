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
    let n = scan.token::<usize>();
    let k = scan.token::<usize>();
    let mut g = vec![Vec::new(); n];
    for i in 1..n {
        let fa = scan.token::<usize>()-1;
        g[fa].push(i)
    }
    let z:Vec<usize> = (0..n).collect();
    let ans = z.partition_point( |&x| {
        if x == 0{
            return n != 0;
        }
        let mut cnt = vec![0;1];

        fn dfs(u :usize,g:& Vec<Vec<usize>>,x:usize,cnt:&mut Vec<usize>) -> usize {
            let mut h = 0;
            for &v in g[u].iter() {
                let p = dfs(v,g,x,cnt);
                if p == x && u > 0{
                    cnt[0] += 1;
                }else{
                    h = h.max(p)
                }
            }
            return h + 1;
        }
        dfs(0,&g,x,&mut cnt);
        return cnt[0] > k;
    });
    writeln!(out, "{}", ans).ok();
}

pub fn run(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let t = scan.token::<usize>();
    for _ in 0..t {
        solve(scan, out)
    }
}
// http://codeforces.com/problemset/problem/1739/D
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步
