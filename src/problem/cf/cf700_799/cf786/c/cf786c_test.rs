use super::cf786c::Scanner;
use super::cf786c::run;
#[cfg(test)]
mod cf786c {
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
1 3 4 3 3",
                    "4 2 1 1 1
"
                );
test_macro!(case2,
                    b"8
1 5 7 8 1 7 6 1",
                    "8 4 3 2 1 1 1 1
"
                );
}           
// https://codeforces.com/contest/786/problem/C
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步