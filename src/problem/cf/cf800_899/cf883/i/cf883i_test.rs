use super::cf883i::Scanner;
use super::cf883i::run;
#[cfg(test)]
mod cf883i {
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
                    b"5 2
50 110 130 40 120",
                    "20
"
                );
test_macro!(case2,
                    b"4 1
2 3 4 1",
                    "0
"
                );
}           
// https://codeforces.com/problemset/problem/883/I
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步