use super::jsc2021_f::Scanner;
use super::jsc2021_f::run;
#[cfg(test)]
mod jsc2021_f {
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
                    b"2 2 4
1 1 10
2 1 20
2 2 5
1 1 30",
                    "20
50
55
85
"
                );
test_macro!(case2,
                    b"3 3 5
1 3 10
2 1 7
1 3 5
2 2 10
2 1 1",
                    "30
44
31
56
42
"
                );
test_macro!(case3,
                    b"200000 200000 4
2 112219 100000000
1 73821 100000000
2 26402 100000000
1 73821 100000000",
                    "20000000000000
39999900000000
59999800000000
59999800000000
"
                );
}           
// https://atcoder.jp/contests/jsc2021/tasks/jsc2021_f
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步