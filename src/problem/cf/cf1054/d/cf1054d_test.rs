use super::cf1054d::Scanner;
use super::cf1054d::run;
#[cfg(test)]
mod cf1054d {
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
                    b"3 2
1 3 0",
                    "5
"
                );
test_macro!(case2,
                    b"6 3
1 4 4 7 3 4",
                    "19
"
                );
}           
// https://codeforces.com/problemset/problem/1054/D
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步