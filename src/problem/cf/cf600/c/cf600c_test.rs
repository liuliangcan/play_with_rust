use super::cf600c::Scanner;
use super::cf600c::run;
#[cfg(test)]
mod cf600c {
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
                    b"aabc",
                    "abba
"
                );
test_macro!(case2,
                    b"aabcd",
                    "abcba
"
                );
}           
// https://codeforces.com/problemset/problem/600/C
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步