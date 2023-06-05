use super::cf414b::Scanner;
use super::cf414b::run;
#[cfg(test)]
mod cf414b {
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
                    b"3 2",
                    "5
"
                );
test_macro!(case2,
                    b"6 4",
                    "39
"
                );
test_macro!(case3,
                    b"2 1",
                    "2
"
                );
}           
// https://codeforces.com/contest/414/problem/B
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步