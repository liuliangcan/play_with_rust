use super::abc268_f::Scanner;
use super::abc268_f::run;
#[cfg(test)]
mod abc268_f {
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
                    b"3
1X3
59
XXX",
                    "71
"
                );
test_macro!(case2,
                    b"10
X63X395XX
X2XX3X22X
13
3716XXX6
45X
X6XX
9238
281X92
1XX4X4XX6
54X9X711X1",
                    "3010
"
                );
}           
// https://atcoder.jp/contests/abc268/tasks/abc268_f
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步