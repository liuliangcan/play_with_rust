use super::cf1444a::Scanner;
use super::cf1444a::run;
#[cfg(test)]
mod cf1444a {
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
10 4
12 6
179 822",
                    "10
4
179
"
                );
}           
// https://codeforces.com/problemset/problem/1444/A
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步