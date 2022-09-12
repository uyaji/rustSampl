fn main() {
    let mut letters = vec![
        "a","b","c"
    ];

    for letter in letters{
        println!("{}", letter);
        letters.push(letter.clone());    // 個々の文字（letter）をコピーして末尾に追加
    }
}
