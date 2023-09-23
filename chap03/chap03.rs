fn build_vector() -> Vec<i16> {
    let mut v: Vec<i16> = Vec::<i16>::new();
    v.push(10i16);
    v.push(20i16);
    v
}
-----------
fn build_vector() -> Vec<i16> {
    let mut v = Vec::new();
    v.push(10);
    v.push(20);
    v
}
-----------
3.1.1 整数型

assert_eq!(   10_i8 as u16,   10_u16); // in range 範囲に入ってる
assert_eq!( 2525_i8 as u16, 2525_u16); // in range 範囲に入ってる

assert_eq!(   -1_i16 as i32,    -1_i32); // sign-extended 符号拡張
assert_eq!(65535_u16 as i32, 65535_i32); // zero-extended ゼロ拡張

// Conversions that are out of range for the destination
// produce values that are equivalent to the original modulo 2^N,
// where N is the width of the destination in bits. This
// is sometimes called "truncation".
// 変換先の範囲からはみ出す変換は、
// 元の値の2^Nによる剰余となる
// ここでNは変換先のビット数である
// これは切り捨て(truncatioin)と呼ばれる
assert_eq!( 1000_i16 as u8, 232_u8);
assert_eq!(65535_u32 as i16, -1_i16);

assert_eq!(   -1_i8 as u8, 255_u8);
assert_eq!(  255_u8 as i8,  -1_u8);

assert_eq!(2_u16.pow(4), 16);            // exponentiation   指数関数
assert_eq!((-4_i32).abs(), 4);           // absolute value   絶対値
assert_eq!(0b101101_u8.count_ones(), 4); // population count ビットカウント


//以下のコードはコンパイルできない
println!("{}", (-4).abs());

//Rustコンパイラは次のようなエラーを出力する
// error: can't call method `abs` on ambiguous numeric type `{integer}`
// 曖昧な数値型`{integer}`に対して`abs`メソッドを呼び出せません

// この問題を解決するには, 末尾に型を指定するか、特定の型を関数を使うようにして
// 型を明示すればいい
println!("{}", (-4_i32).abs());
println!("{}", i32::abs(-4));


3.1.2 チェック付き演算、ラップ演算、飽和演算、オーバーフロー演算

let mut i = 1;
loop {
    i *= 10; // panic: attempt to multiply with overflow パニック:乗算でオーバーフローさせる
             // (but only in debug builds!) (ただしデバッグビルドの場合だけ)
}

let mut i: i32 = 1;
loop {
    // panic: multiplication overflowed (in any build)
    i = i.checked_mul(10).expect("multiplication overflowed");
}

チェック付き演算
// The sum of 10 and 20 can be represented as a u8.
assert_eq!(10_u8.checked_add(20), Some(30));

// Unfortunately, the sum of 100 and 200 cannot.
assert_eq!(100_u8.checked_add(200), None);

// Do the addition; panic if it overflows.
let sum = x.checked_add(y).unwrap();

// Do the addition; panic if it overflows.
let sum = x.checked_add(y).unwrap();

// Oddly, signed division can overflow too, in one particular case.
// A signed n-bit type can represent -2^n-1, but not 2^2-1.
assert_eq!((-128_i8).checked_div(-1), None);


ラップ演算
// The first product can be represented as a u16;
// the second cannnot, so we get 250000 module 2^16
assert_eq!(100_u16.wrapping_mul(200), 20000);
assert_eq!(500_u16.wrapping_mul(500), 53392);

// Operations on signed tpypes may wrap to negative values.
assert_eq!(500_i16.wrapping_mul(500), -12144);

// In bitwise shift operations, the shift distance
// is wrapped to fall within the size of the value.
// So a shift of 17 bits in a 16-bit type is
// a shift of 1.
assert_eq!(6_i16.wrapping_shl(17), 10);


飽和演算
assert_eq!(32760_i16.saturating_add(10), 32767);
assert_eq!((-32760_i16).saturating_sub(10), -32768);


オーバーフロー演算
assert_eq!(255_u8.overflowing_sub(2), (253, false));
assert_eq!(255_u8.overflowing_add(2), (1, true));

// A shift of 17 bits is too large for `u16`, and 17 modulo 16 is 1.
assert_eq!(5_u16.overflowing_shl(17), (10, ture));


3.1.3 浮動小数点数

f32
f64

assert!((-1. / f32::INFINITY).is_sign_negative());
assert_eq!(-f32::MIN, f32::MAX);

assert_eq!(5f32.sqrt() * 5f32.sqrt(), 5); // exactly 5.0, per IEEE
assert_eq!((-1.01f64).floor(), -2.0);

println!("{}", (2.0_f64).sqrt());
println!("{}", f64::sqrt(2.0));




