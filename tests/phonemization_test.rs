use phonetisaurus_g2p::PhonetisaurusModel;

// Trained phonetisaurus model from phonemizer-rs, here for testing
static PHONETISAURUS_MODEL: &[u8] = include_bytes!("model.fst");

#[test]
fn test_phonemization_success() {
    let word = "world";
    let expected_phonemes = "wˈɜɹld";

    let p = PhonetisaurusModel::try_from(PHONETISAURUS_MODEL).unwrap();
    let result = p.phonemize_word(word).unwrap();

    assert_eq!(result.phonemes, expected_phonemes);
}

#[test]
fn test_phonemization_failure() {
    let word = "hello  "; // Whitespaces are not in training dataset of model

    let p = PhonetisaurusModel::try_from(PHONETISAURUS_MODEL).unwrap();
    let result = p.phonemize_word(word);

    assert!(result.is_err())
}

#[test]
fn test_phonemization_empty() {
    let word = "";
    let expected_phonemes = "";

    let p = PhonetisaurusModel::try_from(PHONETISAURUS_MODEL).unwrap();
    let result = p.phonemize_word(word).unwrap();

    assert_eq!(result.phonemes, expected_phonemes);
}

#[test]
fn test_phonemization_unknown_char() {
    let word = "y̆";

    let p = PhonetisaurusModel::try_from(PHONETISAURUS_MODEL).unwrap();
    let result = p.phonemize_word(word);

    assert!(result.is_err())
}
