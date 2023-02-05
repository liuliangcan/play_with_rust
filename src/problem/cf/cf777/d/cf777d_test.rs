use super::cf777d::Scanner;
use super::cf777d::solve;
#[cfg(test)]
mod cf777d {
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
                    b"3
#book
#bigtown
#big",
                    "#b
#big
#big
"
                );
test_macro!(case2,
                    b"3
#book
#cool
#cold",
                    "#book
#co
#cold
"
                );
test_macro!(case3,
                    b"4
#car
#cart
#art
#at",
                    "#
#
#art
#at
"
                );
test_macro!(case4,
                    b"3
#apple
#apple
#fruit",
                    "#apple
#apple
#fruit
"
                );
}           
        