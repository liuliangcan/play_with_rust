use super::abc222_f::Scanner;
use super::abc222_f::run;
#[cfg(test)]
mod abc222_f {
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
1 2 2
2 3 3
1 2 3",
                    "8
6
6
"
                );
test_macro!(case2,
                    b"6
1 2 3
1 3 1
1 4 4
1 5 1
1 6 5
9 2 6 5 3 100",
                    "105
108
106
109
106
14
"
                );
test_macro!(case3,
                    b"6
1 2 1000000000
2 3 1000000000
3 4 1000000000
4 5 1000000000
5 6 1000000000
1 2 3 4 5 6",
                    "5000000006
4000000006
3000000006
3000000001
4000000001
5000000001
"
                );
}           
// https://atcoder.jp/contests/abc222/tasks/abc222_f
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步