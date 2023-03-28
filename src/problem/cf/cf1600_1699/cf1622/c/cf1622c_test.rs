use super::cf1622c::Scanner;
use super::cf1622c::run;
#[cfg(test)]
mod cf1622c {
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
1 10
20
2 69
6 9
7 8
1 2 1 3 1 2 1
10 1
1 2 3 1 2 6 1 6 8 10",
                    "10
0
2
7
"
                );
}           
// https://codeforces.com/contest/1622/problem/C
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步