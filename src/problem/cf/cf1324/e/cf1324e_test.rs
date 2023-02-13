use super::cf1324e::Scanner;
use super::cf1324e::run;
#[cfg(test)]
mod cf1324e {
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
                    b"7 24 21 23
16 17 14 20 20 11 22",
                    "3
"
                );
}           
// https://codeforces.com/contest/1324/problem/E
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步