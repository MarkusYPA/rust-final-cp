use super::*;

fn test_counting(input: &str, expected: &[(&str, u32)]) {
    let mut m: HashMap<String, u32> = counting_words(input);
    for &(k, v) in expected.iter() {
        assert_eq!(
            (k, m.remove(&k.to_string().to_lowercase()).unwrap_or(0)),
            (k, v)
        );
    }
    // may fail with a message that clearly shows all extra pairs in the map
    assert_eq!(m.iter().collect::<Vec<(&String, &u32)>>(), vec![]);
}


#[test]
fn test_simple() {
    test_counting("word", &[("word", 1)]);
    test_counting("hello", &[("hello", 1)]);
    test_counting("hello big world", &[("hello", 1), ("big", 1), ("world", 1)]);
    test_counting("one of each", &[("one", 1), ("of", 1), ("each", 1)]);
    test_counting("Hello, 1, 2 HELLO", &[("Hello", 2), ("1", 1), ("2", 1)]);
    test_counting(
        "Batman, BATMAN, batman, Stop stop",
        &[("batman", 3), ("stop", 2)],
    );
    test_counting(
        " multiple   whitespace",
        &[("multiple", 1), ("whitespace", 1)],
    );
}

#[test]
fn test_count_multiple_occurrences() {
    test_counting(
        "one fish two fish red fish blue fish",
        &[("one", 1), ("fish", 4), ("two", 1), ("red", 1), ("blue", 1)],
    );
}

#[test]
fn test_multi_lines() {
    test_counting(
        "Game\nNight\nTomorrow",
        &[("Game", 1), ("Night", 1), ("Tomorrow", 1)],
    );
}

#[test]
fn test_punctuation() {
    test_counting(
        "keyboard : mouse on the desk : Computer!!&@$%^&",
        &[
            ("keyboard", 1),
            ("mouse", 1),
            ("on", 1),
            ("the", 1),
            ("desk", 1),
            ("Computer", 1),
        ],
    );
}

#[test]
fn with_apostrophes() {
    test_counting(
        "First: don't laugh. Then: don't cry.",
        &[
            ("first", 1),
            ("don't", 2),
            ("laugh", 1),
            ("then", 1),
            ("cry", 1),
        ],
    );
}

#[test]
fn test_apostrophe() {
    test_counting(
        "Joe can't tell between 'large' and large.",
        &[
            ("joe", 1),
            ("can't", 1),
            ("tell", 1),
            ("between", 1),
            ("large", 2),
            ("and", 1),
        ],
    );
}
