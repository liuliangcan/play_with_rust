use super::cf639b::Scanner;
use super::cf639b::run;
#[cfg(test)]
mod cf639b {
    use super::*;

    macro_rules! test_macro {
        ($name:ident, $input:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let output = &mut Vec::new();
                let scan = &mut Scanner::new($input as &[u8]);
                run(scan, output);
                assert_eq!($expected.trim(), std::str::from_utf8(output).unwrap().trim());
            }
        };
    }

    test_macro!(case1,
                    b"5 3 2",
                    "1 2
1 3
3 4
3 5
"
                );
test_macro!(case2,
                    b"8 5 2",
                    "-1
"
                );
test_macro!(case3,
                    b"8 4 2",
                    "4 8
5 7
2 3
8 1
2 1
5 6
1 5
"
                );
}           
// https://codeforces.com/problemset/problem/639/B
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步