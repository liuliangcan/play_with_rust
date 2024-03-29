use super::cf961d::Scanner;
use super::cf961d::run;
#[cfg(test)]
mod cf961d {
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
0 0
0 1
1 1
1 -1
2 2",
                    "YES
"
                );
test_macro!(case2,
                    b"5
0 0
1 0
2 1
1 1
2 3",
                    "NO
"
                );
}           
// https://codeforces.com/contest/961/problem/D
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步