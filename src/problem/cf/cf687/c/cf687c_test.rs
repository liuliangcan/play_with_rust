use super::cf687c::Scanner;
use super::cf687c::run;
#[cfg(test)]
mod cf687c {
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
                    b"6 18
5 6 1 10 12 2",
                    "16
0 1 2 3 5 6 7 8 10 11 12 13 15 16 17 18
"
                );
test_macro!(case2,
                    b"3 50
25 25 50",
                    "3
0 25 50
"
                );
}           
// https://codeforces.com/problemset/problem/687/C
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步