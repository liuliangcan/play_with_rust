use super::cf1272e::Scanner;
use super::cf1272e::run;
#[cfg(test)]
mod cf1272e {
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
                    b"10
4 5 7 6 7 5 4 4 6 4",
                    "1 1 1 2 -1 1 1 3 1 1
"
                );
}           
// https://codeforces.com/problemset/problem/1272/E
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步