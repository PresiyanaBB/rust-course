use solution::*;

#[test]
fn test_basic() {
    let mut fib_iter = FibIter::new();
    fib_iter.next();

    let mut rev_fib_iter: RevFibIter = fib_iter.rev();
    rev_fib_iter.next();

    let words: Vec<String> = fib_split("Fibonacci words!");
    assert_eq!(words, &["F", "i", "bo", "nac", "ci wo", "rds!"]);

    let (words, rest) = fib_split_n("Lorem ipsum dolor sit amet.", 6);
    assert_eq!(words, &["L", "o", "re", "m i", "psum ", "dolor si"]);
    assert_eq!(rest, "t amet.");

    let (words, rest) = fib_split_n_symmetric("Lorem ipsum dolor sit amet.", 5);
    assert_eq!(
        words,
        &["L", "o", "re", "m i", "psum ", "dolor", " si", "t ", "a", "m"]
    );
    assert_eq!(rest, "et.");
}

#[test]
fn test_fib_split() {
    let words_eng = fib_split("Dumite na Fibonacci");
    assert_eq!(words_eng, &["D", "u", "mi", "te ", "na Fi", "bonacci"]);

    let words_bg = fib_split("Думите на Фибоначчи");
    assert_eq!(words_bg, &["Д", "у", "ми", "те ", "на Фи", "боначчи"]);
}

#[test]
fn test_fib_split_n() {
    let words_eng = fib_split_n("Lorem ipsum dolor sit amet.", 6);
    assert_eq!(words_eng.0, &["L", "o", "re", "m i", "psum ", "dolor si"]);
    assert_eq!(words_eng.1, "t amet.");

    let words_bg = fib_split_n("Лорем ипсум долор сит амет.", 6);
    assert_eq!(words_bg.0, &["Л", "о", "ре", "м и", "псум ", "долор си"]);
    assert_eq!(words_bg.1, "т амет.");
}

#[test]
fn test_fib_split_n_symmetric() {
    let words_eng = fib_split_n_symmetric("Lorem ipsum dolor sit amet.", 5);
    assert_eq!(
        words_eng.0,
        &["L", "o", "re", "m i", "psum ", "dolor", " si", "t ", "a", "m"]
    );
    assert_eq!(words_eng.1, "et.");

    let words_bg = fib_split_n_symmetric("Лорем ипсум долор сит амет.", 5);
    assert_eq!(
        words_bg.0,
        &[
            "Л",
            "о",
            "ре",
            "м и",
            "псум ",
            "долор",
            " си",
            "т ",
            "а",
            "м"
        ]
    );
    assert_eq!(words_bg.1, "ет.");
}
