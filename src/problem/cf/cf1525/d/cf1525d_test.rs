use super::cf1525d::Scanner;
use super::cf1525d::run;
#[cfg(test)]
mod cf1525d {
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
                    b"7
1 0 0 1 0 0 1",
                    "3
"
                );
test_macro!(case2,
                    b"6
1 1 1 0 0 0",
                    "9
"
                );
test_macro!(case3,
                    b"5
0 0 0 0 0",
                    "0
"
                );
}           
// https://codeforces.com/contest/1525/problem/D
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步