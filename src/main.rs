mod problem;

#[allow(unused)]
use std::collections::*;
#[allow(unused)]
use std::io::{BufRead, BufWriter, Write};
use itertools::Itertools;

#[allow(unused)]
fn main() {
    use std::collections::HashMap;

    let map = HashMap::from([
        ("a", 1),
        ("b", 2),
        ("c", 3),
    ]);
    let x = map.values().collect_vec();
    println!("{:?}",x);
    // for val in map.values() {
    //     println!("{:?}",val);
    // }
}

// https://codeforces.com/problemset/problem/1733/D2
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步
