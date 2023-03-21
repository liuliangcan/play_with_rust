use super::cf1325d::Scanner;
use super::cf1325d::run;
#[cfg(test)]
mod cf1325d {
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
                    b"2 4",
                    "2
3 1
"
                );
test_macro!(case2,
                    b"1 3",
                    "3
1 1 1
"
                );
test_macro!(case3,
                    b"8 5",
                    "-1
"
                );
test_macro!(case4,
                    b"0 0",
                    "0
"
                );
}           
// https://codeforces.com/problemset/problem/1325/D
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步