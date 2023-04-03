use super::cf1469c::Scanner;
use super::cf1469c::run;
#[cfg(test)]
mod cf1469c {
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
6 3
0 0 2 5 1 1
2 3
0 2
3 2
3 0 2",
                    "YES
YES
NO
"
                );
}           
// https://codeforces.com/problemset/problem/1469/C
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步