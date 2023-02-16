use super::cf165e::Scanner;
use super::cf165e::run;
#[cfg(test)]
mod cf165e {
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
                    b"2
90 36",
                    "36 90
"
                );
test_macro!(case2,
                    b"4
3 6 3 6",
                    "-1 -1 -1 -1
"
                );
test_macro!(case3,
                    b"5
10 6 9 8 2",
                    "-1 8 2 2 8
"
                );
}           
// https://codeforces.com/problemset/problem/165/e
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步