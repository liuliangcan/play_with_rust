use super::arc111_b::Scanner;
use super::arc111_b::solve;
#[cfg(test)]
mod arc111_b {
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
1 2
1 3
4 2
2 3",
                    "4
"
                );
test_macro!(case2,
                    b"2
111 111
111 111",
                    "1
"
                );
test_macro!(case3,
                    b"12
5 2
5 6
1 2
9 7
2 7
5 5
4 2
6 7
2 2
7 8
9 7
1 8",
                    "8
"
                );
}           
        