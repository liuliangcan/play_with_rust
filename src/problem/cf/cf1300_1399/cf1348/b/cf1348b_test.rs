use super::cf1348b::Scanner;
use super::cf1348b::run;
#[cfg(test)]
mod cf1348b {
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
                    b"4
4 2
1 2 2 1
4 3
1 2 2 1
3 2
1 2 3
4 4
4 3 4 2",
                    "5
1 2 1 2 1
4
1 2 2 1
-1
7
4 3 2 1 4 3 2
"
                );
}           
// https://codeforces.com/problemset/problem/1348/B
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步