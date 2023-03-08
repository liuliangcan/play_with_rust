mod problem;

#[allow(unused)]
use std::collections::*;
#[allow(unused)]
use std::io::{BufRead, BufWriter, Write};
#[allow(unused)]
fn main() {
    // let x:Vec<i32> = (0..10).collect();
    let r = [0,1,2,3,4];
    let y:Vec<i32> = (0..10).map(|x|x).collect::<_>();
    println!("{:?}", y);
}

// https://codeforces.com/problemset/problem/1733/D2
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步
