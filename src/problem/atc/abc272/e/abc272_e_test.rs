use super::abc272_e::Scanner;
use super::abc272_e::solve;
#[cfg(test)]
mod abc272_e {
    use super::*;

    macro_rules! test_macro {
        ($name:ident, $input:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let output = &mut Vec::new();
                let scan = &mut Scanner::new($input as &[u8]);
                solve(scan, output);
                assert_eq!($expected, std::str::from_utf8(output).unwrap());
            }
        };
    }

    test_macro!(case1,
                    b"3 3
-1 -1 -6",
                    "2
2
0
"
                );
test_macro!(case2,
                    b"5 6
-2 -2 -5 -7 -15",
                    "1
3
2
0
0
0
"
                );
}           
        