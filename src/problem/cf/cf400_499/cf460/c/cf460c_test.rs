use super::cf460c::Scanner;
use super::cf460c::run;
#[cfg(test)]
mod cf460c {
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
                    b"6 2 3
2 2 2 2 1 1",
                    "2
"
                );
    test_macro!(case2,
                    b"2 5 1
5 8",
                    "9
"
                );
    test_macro!(case3,
                    b"1 1 1
1",
                    "2
"
                );
}           
// https://codeforces.com/problemset/problem/460/C
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步