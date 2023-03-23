use super::cf1624e::Scanner;
use super::cf1624e::run;
#[cfg(test)]
mod cf1624e {
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
                    b"5
4 8
12340219
20215601
56782022
12300678
12345678
2 3
134
126
123
1 4
1210
1221
4 3
251
064
859
957
054
4 7
7968636
9486033
4614224
5454197
9482268",
                    "3
1 4 1
5 6 2
3 4 3
-1
2
1 2 1
2 3 1
-1
3
1 3 2
5 6 3
3 4 1
"
                );

    test_macro!(case2,
                    b"19

2 5
28829
01530
28607

1 1
3
6

4 6
815461
471031
102617
030761
118667

4 1
0
9
3
8
8

2 1
2
8
2

1 3
762
159

4 3
251
064
859
957
054

4 5
06452
42611
14299
27445
93014

2 1
0
8
7

2 2
98
35
47

1 1
6
1

1 7
7640187
9522496

3 2
45
36
04
85

4 2
60
21
75
84
10

2 1
8
9
0

4 5
18307
62543
08156
95228
04936

2 4
0656
2959
1836

4 2
37
50
03
17
28

1 1
7
8",
                    "-1
-1
-1
-1
-1
-1
-1
-1
-1
-1
-1
-1
-1
-1
-1
-1
-1
-1
-1
"
                );
}           
// https://codeforces.com/problemset/problem/1624/E
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步