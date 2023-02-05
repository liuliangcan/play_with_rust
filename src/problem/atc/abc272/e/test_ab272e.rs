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

    test_macro!(
        case1,
        b"3 3
-1 -1 -6",
        "2
2
0
"
    );
    test_macro!(
        case2,
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
    test_macro!(
        case3,
        b"79 6
24 -18 38 -116 -142 -121 -241 -24 -1 9 -63 56 -929 -447 -1077 28 -938 -1249 -846 -166 -1227 -1052 -1376 -602 -1547 -1380 -147 -1360 -2271 -1273 -1143 -2399 -617 -1912 -453 -1254 -721 -882 -2597 -2040 -2968 -1552 -1614 -260 -3387 -2344 -1043 -474 -940 -3595 -2682 -1692 -1119 -670 -3622 -938 -1122 -3082 -3548 -2990 -3683 -2598 -4095 -4146 -3451 -238 -1122 -4435 -2035 -3583 -2891 58 -3662 -1497 -1800 -4594 -2721 -326 -5476",
        "0
0
1
0
0
0
"
    );
}