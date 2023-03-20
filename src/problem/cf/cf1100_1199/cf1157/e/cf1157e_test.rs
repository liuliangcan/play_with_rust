use super::cf1157e::Scanner;
use super::cf1157e::run;
#[cfg(test)]
mod cf1157e {
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
                    b"4
0 1 2 1
3 2 1 1",
                    "1 0 0 2
"
                );
test_macro!(case2,
                    b"7
2 5 1 5 3 4 3
2 4 3 5 6 5 1",
                    "0 0 0 1 0 2 4
"
                );
}           
// https://codeforces.com/problemset/problem/1157/E
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步