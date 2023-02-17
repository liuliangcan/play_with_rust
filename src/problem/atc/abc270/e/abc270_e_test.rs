use super::abc270_e::Scanner;
use super::abc270_e::run;
#[cfg(test)]
mod abc270_e {
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
                    b"3 3
1 3 0",
                    "0 1 0
"
                );
test_macro!(case2,
                    b"2 1000000000000
1000000000000 1000000000000",
                    "500000000000 500000000000
"
                );
}           
// https://atcoder.jp/contests/abc270/tasks/abc270_e
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步