use core::f64;

fn main() {
    let exp = "6.1 5.2 4.3 * + 3.4 2.5 / 1.6 * -";

    let ans = rpn(exp);

    debug_assert_eq!("26.2840", format!("{:.4}", ans));

    println!("{} = {:.4}", exp, ans);
}

fn rpn(exp: &str) -> f64 {
    let mut stack: Vec<f64> = Vec::new();

    for token in exp.split_whitespace() {
        if let Ok(num) = token.parse::<f64>() {
            // `token.parse::<f64>()` この書き方どういう文法なのかがよくわからない
            stack.push(num);
        } else {
            match token {
                // 数値以外 = オペレーターの場合。
                // クロージャを作って渡す。なぜこのようなインタフェースにしているのか、必然性はわからない
                // 高階関数使えば汎用関数1つで済むでしょう？っていう設計意図かなという気はしている
                "+" => apply2(&mut stack, |x, y| x + y),
                "-" => apply2(&mut stack, |x, y| x - y),
                "*" => apply2(&mut stack, |x, y| x * y),
                "/" => apply2(&mut stack, |x, y| x / y),
                _ => panic!("Unknown operator"),
            }
        }
    }

    stack.pop().expect("Stack underflow")
}

fn apply2<F>(stack: &mut Vec<f64>, fun: F) where F: Fn(f64, f64) -> f64 {
    // where 以降の `F: Fn(f64, f64)` はトレイト境界。ジェネリクスで宣言した型Fに対する制約を与える
    if let (Some(y), Some(x)) = (stack.pop(), stack.pop()) {
        // 疑問: どうして 'Some' というものが必要なのか？
        // 
        // https://doc.rust-jp.rs/rust-by-example-ja/std/option.html
        // Result における Ok のようなもので、Option（列挙型）を扱う場合は Some を使う、ということらしい
        // Some(value) のバインドができない状況で if let が else 節に分岐する
        let z = fun(x, y);

        stack.push(z);
    } else {
        // 疑問: ここでの else 節はどのような状況を示している？？
        panic!("Stack underflow");
    }
}