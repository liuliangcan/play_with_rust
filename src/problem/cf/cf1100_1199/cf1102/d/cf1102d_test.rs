use super::cf1102d::Scanner;
use super::cf1102d::run;
#[cfg(test)]
mod cf1102d {
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
                    b"3
121",
                    "021
"
                );
test_macro!(case2,
                    b"6
000000",
                    "001122
"
                );
test_macro!(case3,
                    b"6
211200",
                    "211200
"
                );
test_macro!(case4,
                    b"6
120110",
                    "120120
"
                );
}           
// https://codeforces.com/problemset/problem/1102/D
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步