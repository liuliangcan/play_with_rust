use super::abc270_f::Scanner;
use super::abc270_f::run;
#[cfg(test)]
mod abc270_f {
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
                    b"4 2
1 20 4 7
20 2 20 3
1 3 5
1 4 6",
                    "16
"
                );
test_macro!(case2,
                    b"3 1
1 1 1
10 10 10
1 2 100",
                    "3
"
                );
test_macro!(case3,
                    b"7 8
35 29 36 88 58 15 25
99 7 49 61 67 4 57
2 3 3
2 5 36
2 6 89
1 6 24
5 7 55
1 3 71
3 4 94
5 6 21",
                    "160
"
                );
}           
// https://atcoder.jp/contests/abc270/tasks/abc270_f
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步