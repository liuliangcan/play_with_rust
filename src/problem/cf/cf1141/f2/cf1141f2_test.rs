use super::cf1141f2::Scanner;
use super::cf1141f2::solve;
#[cfg(test)]
mod cf1141f2 {
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
                    b"7
4 1 2 2 1 5 3",
                    "3
7 7
2 3
4 5
"
                );
test_macro!(case2,
                    b"11
-5 -4 -3 -2 -1 0 1 2 3 4 5",
                    "2
3 4
1 1
"
                );
test_macro!(case3,
                    b"4
1 1 1 1",
                    "4
4 4
1 1
2 2
3 3
"
                );
}           
        