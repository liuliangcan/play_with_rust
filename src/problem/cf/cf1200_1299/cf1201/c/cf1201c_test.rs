use super::cf1201c::Scanner;
use super::cf1201c::run;
#[cfg(test)]
mod cf1201c {
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
                    b"3 2
1 3 5",
                    "5
"
                );
test_macro!(case2,
                    b"5 5
1 2 1 1 1",
                    "3
"
                );
test_macro!(case3,
                    b"7 7
4 1 2 4 3 4 4",
                    "5
"
                );
}           
// https://codeforces.com/problemset/problem/1201/C
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步