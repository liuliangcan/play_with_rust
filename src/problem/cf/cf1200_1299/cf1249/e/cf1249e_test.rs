use super::cf1249e::Scanner;
use super::cf1249e::run;
#[cfg(test)]
mod cf1249e {
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
                    b"10 2
7 6 18 6 16 18 1 17 17
6 9 3 10 9 1 10 1 5",
                    "0 7 13 18 24 35 36 37 40 45
"
                );
test_macro!(case2,
                    b"10 1
3 2 3 1 3 3 1 4 1
1 2 3 4 4 1 2 1 3",
                    "0 2 4 7 8 11 13 14 16 17
"
                );
}           
// https://codeforces.com/problemset/problem/1249/E
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步