use super::cf708b::Scanner;
use super::cf708b::run;
#[cfg(test)]
mod cf708b {
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

//     test_macro!(case1,
//                     b"1 2 3 4",
//                     "Impossible
// "
//                 );
//     test_macro!(case2,
//                     b"1 2 2 1",
//                     "0110
// "
//                 );
    test_macro!(case3,
                    b"499928010 820999488 178951395 499991253",
                    "0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
"
                );
}           
// https://codeforces.com/problemset/problem/708/B
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步