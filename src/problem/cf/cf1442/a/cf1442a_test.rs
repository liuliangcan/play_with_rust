use super::cf1442a::Scanner;
use super::cf1442a::solve;
#[cfg(test)]
mod cf1442a {
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
                    b"4
3
1 2 1
5
11 7 9 6 8
5
1 3 1 3 1
4
5 2 1 10",
                    "YES
YES
NO
YES
"
                );
}           
        