use super::cf1661c::Scanner;
use super::cf1661c::run;
#[cfg(test)]
mod cf1661c {
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
3
1 2 4
5
4 4 3 5 5
7
2 5 4 8 3 7 4",
                    "4
3
16
"
                );
}           
// https://codeforces.com/contest/1661/problem/C
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步