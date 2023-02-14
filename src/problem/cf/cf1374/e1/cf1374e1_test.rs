use super::cf1374e1::Scanner;
use super::cf1374e1::run;
#[cfg(test)]
mod cf1374e1 {
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
                    b"8 4
7 1 1
2 1 1
4 0 1
8 1 1
1 0 1
1 1 1
1 0 1
3 0 0",
                    "18
"
                );
test_macro!(case2,
                    b"5 2
6 0 0
9 0 0
1 0 1
2 1 1
5 1 0",
                    "8
"
                );
test_macro!(case3,
                    b"5 3
3 0 0
2 1 0
3 1 0
5 0 1
3 0 1",
                    "-1
"
                );
}           
// https://codeforces.com/contest/1374/problem/E1
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步