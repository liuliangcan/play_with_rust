use super::cf1555d::Scanner;
use super::cf1555d::run;
#[cfg(test)]
mod cf1555d {
    use super::*;

    macro_rules! test_macro {
        ($name:ident, $input:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let output = &mut Vec::new();
                let scan = &mut Scanner::new($input as &[u8]);
                run(scan, output);
                assert_eq!($expected.trim(), std::str::from_utf8(output).unwrap().trim());
            }
        };
    }

    test_macro!(case1,
                    b"5 4
baacb
1 3
1 5
4 5
2 3",
                    "1
2
0
1
"
                );
}           
// https://codeforces.com/problemset/problem/1555/D
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步