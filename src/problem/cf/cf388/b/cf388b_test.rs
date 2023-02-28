use super::cf388b::Scanner;
use super::cf388b::run;
#[cfg(test)]
mod cf388b {
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
                    b"2",
                    "4
NNYY
NNYY
YYNN
YYNN
"
                );
test_macro!(case2,
                    b"9",
                    "8
NNYYYNNN
NNNNNYYY
YNNNNYYY
YNNNNYYY
YNNNNYYY
NYYYYNNN
NYYYYNNN
NYYYYNNN
"
                );
test_macro!(case3,
                    b"1",
                    "2
NY
YN
"
                );
}           
// https://codeforces.com/problemset/problem/388/B
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步