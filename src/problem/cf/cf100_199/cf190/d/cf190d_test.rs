use super::cf190d::Scanner;
use super::cf190d::run;
#[cfg(test)]
mod cf190d {
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
                    b"4 2
1 2 1 2",
                    "3
"
                );
test_macro!(case2,
                    b"5 3
1 2 1 1 3",
                    "2
"
                );
test_macro!(case3,
                    b"3 1
1 1 1",
                    "6
"
                );
}           
// https://codeforces.com/problemset/problem/190/D
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步