use super::cf547b::Scanner;
use super::cf547b::run;
#[cfg(test)]
mod cf547b {
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
                    b"10
1 2 3 4 5 4 3 2 1 6",
                    "6 4 4 3 3 2 2 1 1 1
"
                );
}           
// https://codeforces.com/problemset/problem/547/B
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步