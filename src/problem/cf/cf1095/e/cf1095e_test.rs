use super::cf1095e::Scanner;
use super::cf1095e::run;
#[cfg(test)]
mod cf1095e {
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
                    b"6
(((())",
                    "3
"
                );
test_macro!(case2,
                    b"6
()()()",
                    "0
"
                );
test_macro!(case3,
                    b"1
)",
                    "0
"
                );
test_macro!(case4,
                    b"8
)))(((((",
                    "0
"
                );
}           
// https://codeforces.com/problemset/problem/1095/E
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步