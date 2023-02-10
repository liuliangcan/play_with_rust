use super::cf1361b::Scanner;
use super::cf1361b::run;
#[cfg(test)]
mod cf1361b {
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
5 2
2 3 4 4 3
3 1
2 10 1000
4 5
0 1 1 100
1 8
89",
                    "4
1
146981438
747093407
"
                );
}           
// https://codeforces.com/problemset/problem/1361/B
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步