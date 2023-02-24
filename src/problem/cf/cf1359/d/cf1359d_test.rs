use super::cf1359d::Scanner;
use super::cf1359d::run;
#[cfg(test)]
mod cf1359d {
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
                    b"5
5 -2 10 -1 4",
                    "6
"
                );
test_macro!(case2,
                    b"8
5 2 5 3 -30 -30 6 9",
                    "10
"
                );
test_macro!(case3,
                    b"3
-10 6 -15",
                    "0
"
                );
}           
// https://codeforces.com/contest/1359/problem/D
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步