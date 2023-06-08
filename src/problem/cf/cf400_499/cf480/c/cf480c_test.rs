use super::cf480c::Scanner;
use super::cf480c::run;
#[cfg(test)]
mod cf480c {
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
                    b"5 2 4 1",
                    "2
"
                );
test_macro!(case2,
                    b"5 2 4 2",
                    "2
"
                );
test_macro!(case3,
                    b"5 3 4 1",
                    "0
"
                );
}           
// https://codeforces.com/contest/480/problem/C
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步