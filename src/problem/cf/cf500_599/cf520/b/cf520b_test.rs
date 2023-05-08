use super::cf520b::Scanner;
use super::cf520b::run;
#[cfg(test)]
mod cf520b {
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
                    b"4 6",
                    "2
"
                );
test_macro!(case2,
                    b"10 1",
                    "9
"
                );
}           
// https://codeforces.com/problemset/problem/520/B
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步