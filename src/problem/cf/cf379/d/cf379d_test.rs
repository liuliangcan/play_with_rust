use super::cf379d::Scanner;
use super::cf379d::run;
#[cfg(test)]
mod cf379d {
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
                    b"3 2 2 2",
                    "AC
AC
"
                );
test_macro!(case2,
                    b"3 3 2 2",
                    "Happy new year!
"
                );
test_macro!(case3,
                    b"3 0 2 2",
                    "AA
AA
"
                );
test_macro!(case4,
                    b"4 3 2 1",
                    "Happy new year!
"
                );
test_macro!(case5,
                    b"4 2 2 1",
                    "Happy new year!
"
                );
}           
// https://codeforces.com/problemset/problem/379/D
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步