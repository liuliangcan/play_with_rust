use super::cf382c::Scanner;
use super::cf382c::run;
#[cfg(test)]
mod cf382c {
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
                    b"3
4 1 7",
                    "2
-2 10
"
                );
test_macro!(case2,
                    b"1
10",
                    "-1
"
                );
test_macro!(case3,
                    b"4
1 3 5 9",
                    "1
7
"
                );
test_macro!(case4,
                    b"4
4 3 4 5",
                    "0
"
                );
    test_macro!(case5,
                    b"2
2 4",
                    "3
0 3 6
"
                );
    test_macro!(case6,
                    b"2
3 3",
                    "1
3
"
                );
}           
// https://codeforces.com/problemset/problem/382/C
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步