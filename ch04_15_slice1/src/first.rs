
#[allow(dead_code)]
pub fn sample() {
    println!("first.rs");

    // 配列
    let a1 = ['a', 'b', 'c', 'd']; // 参照元のデータ, [char, 4]型
    println!(" a1: {:?}", a1);

    print_info("&a1[..]", &a1[..]);     // &[char]型,全要素のスライス
    print_info("&a1", &a1);             // 同上
    print_info("&a1[1..3]", &a1[1..3]); // bとcを要素とする長さ2のスライス

    // ベクタ
    let v1 = vec!['e', 'f', 'g', 'h']; // 参照元のデータ, Vec<char>型
    println!("\n v1: {:?}", v1);

    print_info("&v1[..]", &v1[..]);     // &[char]型,全要素のスライス
    print_info("&v1", &v1);             // 同上
    print_info("&v1[1..3]", &v1[1..3]); // &[char]型,fとgを要素とする長さ2のスライス

    // ↑のサンプルでのbegin.. ..endでは end の値が含まれていない点に注意
    // アクセスされるのは 1から2の要素
}


// &[char]型のスライスを引数に撮り、その情報を表示する
fn print_info(  name: &str,
                s1: &[char]) {
    println!(" {:9} - {}, {:?}, {:?}, {:?}",
        name,       //
        s1.len(),   // 長さ( バイト長 ) usize型
        s1.first(), // 最初の要素       Option<char>型
        s1[1],      // 2番目の要素      char型
        s1.last()   // 最後の要素       Option<char>型
    );
}
