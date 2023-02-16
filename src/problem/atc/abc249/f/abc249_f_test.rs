use super::abc249_f::Scanner;
use super::abc249_f::run;
#[cfg(test)]
mod abc249_f {
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
                    b"5 1
2 4
2 -3
1 2
2 1
2 -3",
                    "3
"
                );
test_macro!(case2,
                    b"1 0
2 -1000000000",
                    "-1000000000
"
                );
test_macro!(case3,
                    b"10 3
2 3
2 -1
1 4
2 -1
2 5
2 -9
2 2
1 -6
2 5
2 -3",
                    "15
"
                );
}           
// https://atcoder.jp/contests/abc249/tasks/abc249_f
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步