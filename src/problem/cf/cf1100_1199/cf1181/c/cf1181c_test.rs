use super::cf1181c::Scanner;
use super::cf1181c::run;
#[cfg(test)]
mod cf1181c {
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
                    b"4 3
aaa
bbb
ccb
ddd",
                    "6
"
                );
test_macro!(case2,
                    b"6 1
a
a
b
b
c
c",
                    "1
"
                );
}           
// https://codeforces.com/problemset/problem/1181/C
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步