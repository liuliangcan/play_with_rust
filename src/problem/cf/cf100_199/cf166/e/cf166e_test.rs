use super::cf166e::Scanner;
use super::cf166e::run;
#[cfg(test)]
mod cf166e {
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
                    b"2",
                    "3
"
                );
    test_macro!(case2,
                    b"4",
                    "21
"
                );
    test_macro!(case3,
                    b"10000000",
                    "192336614
"
                );
}           
// https://codeforces.com/problemset/problem/166/E
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步