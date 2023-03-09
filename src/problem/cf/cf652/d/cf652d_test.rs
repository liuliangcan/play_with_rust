use super::cf652d::Scanner;
use super::cf652d::run;
#[cfg(test)]
mod cf652d {
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
1 8
2 3
4 7
5 6",
                    "3
0
1
0
"
                );
    test_macro!(case2,
                    b"3
3 4
1 5
2 6",
                    "0
1
1
"
                );
    test_macro!(case3,
                    b"1
-1000000000 1000000000",
                    "0
"
                );
    test_macro!(case4,
                    b"10
-3 -1
-10 4
0 6
-4 -2
1 3
2 9
5 10
-7 -6
-8 8
-9 7",
                    "0
4
1
0
0
0
0
0
5
5
"
                );
}           
// https://codeforces.com/problemset/problem/652/D
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步