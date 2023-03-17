use super::cf118c::Scanner;
use super::cf118c::run;
#[cfg(test)]
mod cf118c {
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
                    b"6 5
898196",
                    "4
888188
"
                );
test_macro!(case2,
                    b"3 2
533",
                    "0
533
"
                );
    test_macro!(case3,
                    b"10 6
0001112223",
                    "3
0000002223
"
                );
    test_macro!(case4,
                    b"24 5
438088068198972282890781",
                    "0
438088068198972282890781
"
                );
    test_macro!(case5,
                    b"82 80
2119762952003918195325258677229419698255491250839396799769357665825441616335532825",
                    "184
5555555555005555555555555555555555555555555555555555555555555555555555555555555555
"
                );
}           
// https://codeforces.com/problemset/problem/118/C
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步