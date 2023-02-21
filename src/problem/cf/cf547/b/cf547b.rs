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
}
#[allow(unused)]
pub fn solve1(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let mut a = vec![0i32; n];
    let mut ls = vec![-1i32; n];
    let mut rs = vec![n as i32; n];
    let mut st1 = vec![0i32; 0];
    let mut st2 = vec![0i32; 0];
    for i in 0..n {
        a[i] = scan.token::<i32>();
        while st1.len() > 0 && a[*st1.last().unwrap() as usize] >= a[i] {
            st1.pop();
        }
        if st1.len() > 0 {
            ls[i] = *st1.last().unwrap();
        }
        st1.push(i as i32);
        while st2.len() > 0 && a[*st2.last().unwrap() as usize] > a[i] {
            rs[st2.pop().unwrap() as usize] = i as i32;
        }
        st2.push(i as i32);
    }
    let mut ans = vec![0i32; n];
    for i in 0..n {
        let len = (rs[i] - ls[i] - 1 - 1) as usize;
        ans[len] = ans[len].max(a[i])
    }
    for i in (0..n - 1).rev() {
        ans[i] = ans[i].max(ans[i + 1])
    }
    for v in ans {
        write!(out, "{} ", v).ok();
    }
}

pub fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let mut a = vec![0i32; n];
    for i in 0..n {
        a[i] = scan.token::<i32>()
    }
    let (l, r) = MonoStack::new(&a).get_true_bound_as_min();
    let mut ans = vec![0i32; n];
    for i in 0..n {
        let len = (r[i] - l[i] - 2) as usize;
        ans[len] = ans[len].max(a[i])
    }
    for i in (0..n - 1).rev() {
        ans[i] = ans[i].max(ans[i + 1])
    }
    for v in ans {
        write!(out, "{} ", v).ok();
    }
}

pub fn run(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    solve(scan, out)
}
// https://codeforces.com/problemset/problem/547/B
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步

#[allow(unused)]
/// 单调栈计算每个值作为最大/最小值的管辖范围
pub struct MonoStack {
    a: Vec<i32>,
}

impl MonoStack {
    /// 不知道怎么存引用，进来后会clone一份a
    pub fn new(a: &Vec<i32>) -> Self {
        Self { a: a.clone() }
    }
    /// 单调递增栈计算:每个值作为最小值前后能到达的边界(左右第一个比它小的值)
    ///     这里向左会越过相同值，向右会在相同值停下来(防止重复计算区间)
    /// 初始值为 -1/n
    pub fn get_bound_as_min_left_over_and_right_stop(&self) -> (Vec<i32>, Vec<i32>) {
        let a = &(self.a);
        let n = a.len();
        let mut l = vec![-1i32; n];
        let mut r = vec![n as i32; n];
        let mut st = Vec::new();
        for (i, &v) in a.iter().enumerate() {
            while st.len() > 0 && a[*st.last().unwrap() as usize] >= v {
                r[st.pop().unwrap() as usize] = i as i32;
            }
            if st.len() > 0 {
                l[i] = *st.last().unwrap();
            }
            st.push(i as i32);
        }
        return (l, r);
    }
    /// 单调递增栈计算:每个值作为最小值前后能到达的边界(左右第一个比它小的值)
    ///     这里向左会在相同值停下来，向右会越过相同值(防止重复计算区间)
    /// 初始值为 -1/n
    pub fn get_bound_as_min_left_stop_and_right_over(&self) -> (Vec<i32>, Vec<i32>) {
        let a = &self.a;
        let n = a.len();
        let mut l = vec![-1i32; n];
        let mut r = vec![n as i32; n];
        let mut st = Vec::new();
        for (i, &v) in a.iter().enumerate() {
            while st.len() > 0 && a[*st.last().unwrap() as usize] > v {
                r[st.pop().unwrap() as usize] = i as i32;
            }
            if st.len() > 0 {
                l[i] = *st.last().unwrap();
            }
            st.push(i as i32);
        }
        return (l, r);
    }
    /// 获取每个值作为最小值能触达的边界(越过相同值)，即两侧第一个小于它的数
    pub fn get_true_bound_as_min(&self) -> (Vec<i32>, Vec<i32>) {
        let (l, _) = self.get_bound_as_min_left_over_and_right_stop();
        let (_, r) = self.get_bound_as_min_left_stop_and_right_over();
        return (l, r);
    }
    /// 单调递增栈计算:每个值作为最大值前后能到达的边界(左右第一个比它大的值)
    ///     这里向左会越过相同值，向右会在相同值停下来(防止重复计算区间)
    /// 初始值为 -1/n
    pub fn get_bound_as_max_left_over_and_right_stop(&self) -> (Vec<i32>, Vec<i32>) {
        let a = &(self.a);
        let n = a.len();
        let mut l = vec![-1i32; n];
        let mut r = vec![n as i32; n];
        let mut st = Vec::new();
        for (i, &v) in a.iter().enumerate() {
            while st.len() > 0 && a[*st.last().unwrap() as usize] <= v {
                r[st.pop().unwrap() as usize] = i as i32;
            }
            if st.len() > 0 {
                l[i] = *st.last().unwrap();
            }
            st.push(i as i32);
        }
        return (l, r);
    }
    /// 单调递增栈计算:每个值作为最大值前后能到大的边界(左右第一个比它大的值)
    ///     这里向左会在相同值停下来，向右会越过相同值(防止重复计算区间)
    /// 初始值为 -1/n
    pub fn get_bound_as_max_left_stop_and_right_over(&self) -> (Vec<i32>, Vec<i32>) {
        let a = &self.a;
        let n = a.len();
        let mut l = vec![-1i32; n];
        let mut r = vec![n as i32; n];
        let mut st = Vec::new();
        for (i, &v) in a.iter().enumerate() {
            while st.len() > 0 && a[*st.last().unwrap() as usize] < v {
                r[st.pop().unwrap() as usize] = i as i32;
            }
            if st.len() > 0 {
                l[i] = *st.last().unwrap();
            }
            st.push(i as i32);
        }
        return (l, r);
    }
    /// 获取每个值作为最大值能触达的边界(越过相同值)，即两侧第一个大于它的数
    pub fn get_true_bound_as_max(&self) -> (Vec<i32>, Vec<i32>) {
        let (l, _) = self.get_bound_as_max_left_over_and_right_stop();
        let (_, r) = self.get_bound_as_max_left_stop_and_right_over();
        return (l, r);
    }
}
