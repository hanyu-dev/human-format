//! Tests

use human_format_next::Formatter;

macro_rules! test_formatter {
    (
        $formatter:expr;
        $($number:expr => $ideal:expr),*
    ) => {
        $(
            assert_eq!(
                $ideal,
                $formatter.format($number).to_string(),
                concat!("Format `", stringify!($number), "` error")
            );
        )*
    };
}

#[test]
fn test_si() {
    // "K", "M", "G", "T", "P", "E", "Z", "Y"
    test_formatter! {
        Formatter::SI.with_decimals::<2>();
        0 => "0",
        1 => "1",
        -1 => "-1",
        2 => "2",
        -2 => "-2",
        999 => "999",
        -999 => "-999",

        // K
        1_000 => "1.00 K",
        -1_000 => "-1.00 K",
        1_009 => "1.00 K",
        -1_009 => "-1.00 K",
        9_009 => "9.00 K",
        -9_009 => "-9.00 K",
        9_999 => "9.99 K",
        -9_999 => "-9.99 K",
        99_009 => "99.00 K",
        -99_009 => "-99.00 K",
        99_999 => "99.99 K",
        -99_999 => "-99.99 K",
        999_009 => "999.00 K",
        -999_009 => "-999.00 K",
        999_999 => "999.99 K",
        -999_999 => "-999.99 K",

        // M
        1_000_000 => "1.00 M",
        -1_000_000 => "-1.00 M",
        1_009_999 => "1.00 M",
        -1_009_999 => "-1.00 M",
        9_009_999 => "9.00 M",
        -9_009_999 => "-9.00 M",
        9_999_999 => "9.99 M",
        -9_999_999 => "-9.99 M",
        99_009_999 => "99.00 M",
        -99_009_999 => "-99.00 M",
        99_999_999 => "99.99 M",
        -99_999_999 => "-99.99 M",
        999_009_999 => "999.00 M",
        -999_009_999 => "-999.00 M",
        999_999_999 => "999.99 M",
        -999_999_999 => "-999.99 M",

        // G
        1_000_000_000 => "1.00 G",
        -1_000_000_000 => "-1.00 G",
        1_009_999_999 => "1.00 G",
        -1_009_999_999 => "-1.00 G",
        9_009_999_999_isize => "9.00 G",
        -9_009_999_999_isize => "-9.00 G",
        9_999_999_999_isize => "9.99 G",
        -9_999_999_999_isize => "-9.99 G",
        99_009_999_999_isize => "99.00 G",
        -99_009_999_999_isize => "-99.00 G",
        99_999_999_999_isize => "99.99 G",
        -99_999_999_999_isize => "-99.99 G",
        999_009_999_999_isize => "999.00 G",
        -999_009_999_999_isize => "-999.00 G",
        999_999_999_999_isize => "999.99 G",
        -999_999_999_999_isize => "-999.99 G",

        // T
        1_000_000_000_000_isize => "1.00 T",
        -1_000_000_000_000_isize => "-1.00 T",
        1_009_999_999_999_isize => "1.00 T",
        -1_009_999_999_999_isize => "-1.00 T",
        9_009_999_999_999_isize => "9.00 T",
        -9_009_999_999_999_isize => "-9.00 T",
        9_999_999_999_999_isize => "9.99 T",
        -9_999_999_999_999_isize => "-9.99 T",
        99_009_999_999_999_isize => "99.00 T",
        -99_009_999_999_999_isize => "-99.00 T",
        99_999_999_999_999_isize => "99.99 T",
        -99_999_999_999_999_isize => "-99.99 T",
        999_009_999_999_999_isize => "999.00 T",
        -999_009_999_999_999_isize => "-999.00 T",
        999_999_999_999_999_isize => "999.99 T",
        -999_999_999_999_999_isize => "-999.99 T",

        // P
        1_000_000_000_000_000_isize => "1.00 P",
        -1_000_000_000_000_000_isize => "-1.00 P",
        1_009_999_999_999_999_isize => "1.00 P",
        -1_009_999_999_999_999_isize => "-1.00 P",
        9_009_999_999_999_999_isize => "9.00 P",
        -9_009_999_999_999_999_isize => "-9.00 P",
        9_999_999_999_999_999_isize => "9.99 P",
        -9_999_999_999_999_999_isize => "-9.99 P",
        99_009_999_999_999_999_isize => "99.00 P",
        -99_009_999_999_999_999_isize => "-99.00 P",
        99_999_999_999_999_999_isize => "99.99 P",
        -99_999_999_999_999_999_isize => "-99.99 P",
        999_009_999_999_999_999_isize => "999.00 P",
        -999_009_999_999_999_999_isize => "-999.00 P",
        999_999_999_999_999_999_isize => "999.99 P",
        -999_999_999_999_999_999_isize => "-999.99 P",

        // E
        1_000_000_000_000_000_000_isize => "1.00 E",
        -1_000_000_000_000_000_000_isize => "-1.00 E",
        1_009_999_999_999_999_999_isize => "1.00 E",
        -1_009_999_999_999_999_999_isize => "-1.00 E",
        9_009_999_999_999_999_999_isize => "9.00 E",
        -9_009_999_999_999_999_999_isize => "-9.00 E",
        9_999_999_999_999_999_999_i128 => "9.99 E",
        -9_999_999_999_999_999_999_i128 => "-9.99 E",
        99_009_999_999_999_999_999_i128 => "99.00 E",
        -99_009_999_999_999_999_999_i128 => "-99.00 E",
        99_999_999_999_999_999_999_i128 => "99.99 E",
        -99_999_999_999_999_999_999_i128 => "-99.99 E",
        999_009_999_999_999_999_999_i128 => "999.00 E",
        -999_009_999_999_999_999_999_i128 => "-999.00 E",
        999_999_999_999_999_999_999_i128 => "999.99 E",
        -999_999_999_999_999_999_999_i128 => "-999.99 E",

        // Z
        1_000_000_000_000_000_000_000_i128 => "1.00 Z",
        -1_000_000_000_000_000_000_000_i128 => "-1.00 Z",
        1_009_999_999_999_999_999_999_i128 => "1.00 Z",
        -1_009_999_999_999_999_999_999_i128 => "-1.00 Z",
        9_009_999_999_999_999_999_999_i128 => "9.00 Z",
        -9_009_999_999_999_999_999_999_i128 => "-9.00 Z",
        9_999_999_999_999_999_999_999_i128 => "9.99 Z",
        -9_999_999_999_999_999_999_999_i128 => "-9.99 Z",
        99_009_999_999_999_999_999_999_i128 => "99.00 Z",
        -99_009_999_999_999_999_999_999_i128 => "-99.00 Z",
        99_999_999_999_999_999_999_999_i128 => "99.99 Z",
        -99_999_999_999_999_999_999_999_i128 => "-99.99 Z",
        999_009_999_999_999_999_999_999_i128 => "999.00 Z",
        -999_009_999_999_999_999_999_999_i128 => "-999.00 Z",
        999_999_999_999_999_999_999_999_i128 => "999.99 Z",
        -999_999_999_999_999_999_999_999_i128 => "-999.99 Z",

        // Y
        1_000_000_000_000_000_000_000_000_i128 => "1.00 Y",
        -1_000_000_000_000_000_000_000_000_i128 => "-1.00 Y",
        1_009_999_999_999_999_999_999_999_i128 => "1.00 Y",
        -1_009_999_999_999_999_999_999_999_i128 => "-1.00 Y",
        9_009_999_999_999_999_999_999_999_i128 => "9.00 Y",
        -9_009_999_999_999_999_999_999_999_i128 => "-9.00 Y",
        9_999_999_999_999_999_999_999_999_i128 => "9.99 Y",
        -9_999_999_999_999_999_999_999_999_i128 => "-9.99 Y",
        99_009_999_999_999_999_999_999_999_i128 => "99.00 Y",
        -99_009_999_999_999_999_999_999_999_i128 => "-99.00 Y",
        99_999_999_999_999_999_999_999_999_i128 => "99.99 Y",
        -99_999_999_999_999_999_999_999_999_i128 => "-99.99 Y",
        999_009_999_999_999_999_999_999_999_i128 => "999.00 Y",
        -999_009_999_999_999_999_999_999_999_i128 => "-999.00 Y",
        999_999_999_999_999_999_999_999_999_i128 => "999.99 Y",
        -999_999_999_999_999_999_999_999_999_i128 => "-999.99 Y"
    }

    test_formatter! {
        Formatter::<1000, 2>::new(&["K", "M", "G"]);
        0 => "0",
        1 => "1",
        -1 => "-1",
        2 => "2",
        -2 => "-2",
        999 => "999",
        -999 => "-999",

        // K
        1_000 => "1.00 K",
        -1_000 => "-1.00 K",
        1_009 => "1.00 K",
        -1_009 => "-1.00 K",
        9_009 => "9.00 K",
        -9_009 => "-9.00 K",
        9_999 => "9.99 K",
        -9_999 => "-9.99 K",
        99_009 => "99.00 K",
        -99_009 => "-99.00 K",
        99_999 => "99.99 K",
        -99_999 => "-99.99 K",
        999_009 => "999.00 K",
        -999_009 => "-999.00 K",
        999_999 => "999.99 K",
        -999_999 => "-999.99 K",

        // M
        1_000_000 => "1.00 M",
        -1_000_000 => "-1.00 M",
        1_009_999 => "1.00 M",
        -1_009_999 => "-1.00 M",
        9_009_999 => "9.00 M",
        -9_009_999 => "-9.00 M",
        9_999_999 => "9.99 M",
        -9_999_999 => "-9.99 M",
        99_009_999 => "99.00 M",
        -99_009_999 => "-99.00 M",
        99_999_999 => "99.99 M",
        -99_999_999 => "-99.99 M",
        999_009_999 => "999.00 M",
        -999_009_999 => "-999.00 M",
        999_999_999 => "999.99 M",
        -999_999_999 => "-999.99 M",

        // G
        1_000_000_000 => "1.00 G",
        -1_000_000_000 => "-1.00 G",
        1_009_999_999 => "1.00 G",
        -1_009_999_999 => "-1.00 G",
        9_009_999_999_isize => "9.00 G",
        -9_009_999_999_isize => "-9.00 G",
        9_999_999_999_isize => "9.99 G",
        -9_999_999_999_isize => "-9.99 G",
        99_009_999_999_isize => "99.00 G",
        -99_009_999_999_isize => "-99.00 G",
        99_999_999_999_isize => "99.99 G",
        -99_999_999_999_isize => "-99.99 G",
        999_009_999_999_isize => "999.00 G",
        -999_009_999_999_isize => "-999.00 G",
        999_999_999_999_isize => "999.99 G",
        -999_999_999_999_isize => "-999.99 G",

        // T
        1_000_000_000_000_isize => "1.00e12",
        -1_000_000_000_000_isize => "-1.00e12"
    }
}

#[test]
fn test_chinese() {
    // "万", "亿", "兆", "京", "垓", "秭", "穰", "沟"
    test_formatter! {
        Formatter::CHINESE.with_decimals::<2>();
        0 => "0",
        1 => "1",
        -1 => "-1",
        2 => "2",
        -2 => "-2",
        999 => "999",
        -999 => "-999",
        9999 => "9999",
        -9999 => "-9999",

        // 万
        1_0000 => "1.00 万",
        -1_0000 => "-1.00 万",
        1_0099 => "1.00 万",
        -1_0099 => "-1.00 万",

        9_0099 => "9.00 万",
        -9_0099 => "-9.00 万",
        9_9999 => "9.99 万",
        -9_9999 => "-9.99 万",

        99_0099 => "99.00 万",
        -99_0099 => "-99.00 万",
        99_9999 => "99.99 万",
        -99_9999 => "-99.99 万",

        999_0099 => "999.00 万",
        -999_0099 => "-999.00 万",
        999_9999 => "999.99 万",
        -999_9999 => "-999.99 万",

        9999_0099 => "9999.00 万",
        -9999_0099 => "-9999.00 万",
        9999_9999 => "9999.99 万",
        -9999_9999 => "-9999.99 万",

        // 亿
        1_0000_0000 => "1.00 亿",
        -1_0000_0000 => "-1.00 亿",
        1_0099_9999 => "1.00 亿",
        -1_0099_9999 => "-1.00 亿",

        9_0099_9999 => "9.00 亿",
        -9_0099_9999 => "-9.00 亿",
        9_9999_9999 => "9.99 亿",
        -9_9999_9999 => "-9.99 亿",

        99_0099_9999_isize => "99.00 亿",
        -99_0099_9999_isize => "-99.00 亿",
        99_9999_9999_isize => "99.99 亿",
        -99_9999_9999_isize => "-99.99 亿",

        999_0099_9999_isize => "999.00 亿",
        -999_0099_9999_isize => "-999.00 亿",
        999_9999_9999_isize => "999.99 亿",
        -999_9999_9999_isize => "-999.99 亿",

        9999_0099_9999_isize => "9999.00 亿",
        -9999_0099_9999_isize => "-9999.00 亿",
        9999_9999_9999_isize => "9999.99 亿",
        -9999_9999_9999_isize => "-9999.99 亿",

        // 兆
        1_0000_0000_0000_isize => "1.00 兆",
        -1_0000_0000_0000_isize => "-1.00 兆",
        1_0099_9999_9999_isize => "1.00 兆",
        -1_0099_9999_9999_isize => "-1.00 兆",

        9_0099_9999_9999_isize => "9.00 兆",
        -9_0099_9999_9999_isize => "-9.00 兆",
        9_9999_9999_9999_isize => "9.99 兆",
        -9_9999_9999_9999_isize => "-9.99 兆",

        99_0099_9999_9999_isize => "99.00 兆",
        -99_0099_9999_9999_isize => "-99.00 兆",
        99_9999_9999_9999_isize => "99.99 兆",
        -99_9999_9999_9999_isize => "-99.99 兆",

        999_0099_9999_9999_isize => "999.00 兆",
        -999_0099_9999_9999_isize => "-999.00 兆",
        999_9999_9999_9999_isize => "999.99 兆",
        -999_9999_9999_9999_isize => "-999.99 兆",

        9999_0099_9999_9999_isize => "9999.00 兆",
        -9999_0099_9999_9999_isize => "-9999.00 兆",
        9999_9999_9999_9999_isize => "9999.99 兆",
        -9999_9999_9999_9999_isize => "-9999.99 兆",

        // 京
        1_0000_0000_0000_0000_isize => "1.00 京",
        -1_0000_0000_0000_0000_isize => "-1.00 京",
        1_0099_9999_9999_9999_isize => "1.01 京", // f64 limitation
        -1_0099_9999_9999_9999_isize => "-1.01 京",

        9_0099_9999_9999_9999_isize => "9.01 京",
        -9_0099_9999_9999_9999_isize => "-9.01 京",
        9_9999_9999_9999_9999_isize => "10.00 京",
        -9_9999_9999_9999_9999_isize => "-10.00 京",

        99_0099_9999_9999_9999_isize => "99.01 京",
        -99_0099_9999_9999_9999_isize => "-99.01 京",
        99_9999_9999_9999_9999_isize => "100.00 京",
        -99_9999_9999_9999_9999_isize => "-100.00 京",

        999_0099_9999_9999_9999_i128 => "999.01 京",
        -999_0099_9999_9999_9999_i128 => "-999.01 京",
        999_9999_9999_9999_9999_i128 => "1000.00 京",
        -999_9999_9999_9999_9999_i128 => "-1000.00 京",

        9999_0099_9999_9999_9999_i128 => "9999.01 京",
        -9999_0099_9999_9999_9999_i128 => "-9999.01 京",
        9999_9899_9999_9999_9999_i128 => "9999.99 京",
        -9999_9899_9999_9999_9999_i128 => "-9999.99 京",
        9999_9900_0000_0000_0000_i128 => "9999.99 京",
        -9999_9900_0000_0000_0000_i128 => "-9999.99 京"
    }
}
