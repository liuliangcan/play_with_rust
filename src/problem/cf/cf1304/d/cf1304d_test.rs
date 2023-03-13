use super::cf1304d::Scanner;
use super::cf1304d::run;
#[cfg(test)]
mod cf1304d {
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
                    b"3
3 <<
7 >><>><
5 >>><",
                    "1 2 3
1 2 3
5 4 3 7 2 1 6
4 3 1 7 5 2 6
4 3 2 1 5
5 4 2 1 3
"
                );
}           
// http://codeforces.com/problemset/problem/1304/D
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步