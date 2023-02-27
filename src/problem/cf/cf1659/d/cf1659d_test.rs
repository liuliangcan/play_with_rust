use super::cf1659d::Scanner;
use super::cf1659d::run;
#[cfg(test)]
mod cf1659d {
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
                    b"5
4
2 4 2 4
7
0 3 4 2 3 2 7
3
0 0 0
4
0 0 0 4
3
1 2 3",
                    "1 1 0 1 
0 1 1 0 0 0 1 
0 0 0 
0 0 0 1 
1 0 1
"
                );
}           
// https://codeforces.com/problemset/problem/1659/D
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步