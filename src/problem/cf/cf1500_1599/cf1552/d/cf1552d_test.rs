use super::cf1552d::Scanner;
use super::cf1552d::run;
#[cfg(test)]
mod cf1552d {
    use super::*;

    macro_rules! test_macro {
        ($name:ident, $input:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let output = &mut Vec::new();
                let scan = &mut Scanner::new($input as &[u8]);
                run(scan, output);
                assert_eq!($expected, std::str::from_utf8(output).unwrap());
            }
        };
    }

    test_macro!(case1,
                    b"5
5
4 -7 -1 5 10
1
0
3
1 10 100
4
-3 2 10 2
9
25 -171 250 174 152 242 100 -205 -258",
                    "YES
YES
NO
YES
YES
"
                );
}           
// https://codeforces.com/problemset/problem/1552/D
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步