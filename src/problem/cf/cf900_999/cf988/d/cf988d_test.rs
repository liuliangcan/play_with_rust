use super::cf988d::Scanner;
use super::cf988d::run;
#[cfg(test)]
mod cf988d {
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
                    b"6
3 5 4 7 10 12",
                    "3
7 3 5
"
                );
test_macro!(case2,
                    b"5
-1 2 5 8 11",
                    "1
8
"
                );
}           
// https://codeforces.com/contest/988/problem/D
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步