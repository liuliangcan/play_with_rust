use super::arc153_c::Scanner;
use super::arc153_c::run;
#[cfg(test)]
mod arc153_c {
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
                    b"5
-1 1 -1 -1 1",
                    "Yes
-3 -1 4 5 7
"
                );
test_macro!(case2,
                    b"1
-1",
                    "Yes
0
"
                );
test_macro!(case3,
                    b"2
1 -1",
                    "No
"
                );
}           
// https://atcoder.jp/contests/arc153/tasks/arc153_c
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步