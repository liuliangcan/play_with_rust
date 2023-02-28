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
    let mut k = scan.token::<usize>();
    let mut a = vec![0usize;0];
    let mut s = 0usize;
    while k >= 32 {
        let x = (k as f32).powf(0.2) as usize;
        k -= x.pow(5);
        a.push(x);
        s += x;
    }
    fn add(x:usize,y:usize,ans: &mut Vec<Vec<u8>>){
        ans[x][y] = b'Y';
            ans[y][x] = b'Y';
    }
    let n = 2 + s*5+4+k;
    let mut start = 2usize;
    let mut ans = vec![vec![b'N';n];n];
    for x in a{
        for i in 0..4{
            for y in 0..x{
                let l = start + i*x +y;
                for z in 0..x{
                    let r = start + (i+1)*x + z;
                    add(l,r,&mut ans);
                }
            }
        }
        for y in 0..x{
            let l = start + y ;
            let r = start + 4*x + y;
            add(l,0,&mut ans);
            add(r,1,&mut ans);
        }
        start += 5*x;
    }
    for i in start..(start+k){
        add(0,i,&mut ans);
        add(start+k,i,&mut ans);
    }
    start += k;
    for _ in 0..3{
        add(start,start+1,&mut ans);
        start += 1;
    }
    add(1,start,&mut ans);
    writeln!(out, "{}", n).ok();
    for row in ans.iter(){
        writeln!(out,"{}",String::from_utf8_lossy(row)).ok();
    }
}

pub fn run(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {

    solve(scan,out)
}
// https://codeforces.com/problemset/problem/388/B
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步
