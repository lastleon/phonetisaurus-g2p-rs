# Phonetisaurus-G2P
Phonemization in Rust using a finite state transducer (FST) trained with [Phonetisaurus](https://github.com/AdolfVonKleist/Phonetisaurus).

Allows easy usage of a Phonetisaurus-trained FST for grapheme-to-phoneme conversion. Based on [`rustfst`](https://docs.rs/rustfst/latest/rustfst/),
a Rust implementation of FSTs compatible with OpenFST models, and thus also with Phonetisaurus. In theory, this library can be used with all
OpenFST models, but only Phonetisaurus was tested and some details might only be applicable for Phonetisaurus.

Note that the API might slightly change in the future.

## Usage
To use the crate, add the following to your `Cargo.toml`:
```toml
[dependencies]
phonetisaurus-g2p = "0.1.1"
```

Then, you can include the model file in your binary and run it:
```rust
use phonetisaurus_g2p::PhonetisaurusModel;

static PHONETISAURUS_MODEL: &[u8] = include_bytes!("model.fst");

fn main() {
    let phonemizer = PhonetisaurusModel::try_from(PHONETISAURUS_MODEL).unwrap();

    let result = phonemizer.phonemize_word("world").unwrap();
    assert_eq!(result.phonemes, "wˈɜɹld")
}
```


Or load it from disk during runtime:
```rust
use phonetisaurus_g2p::PhonetisaurusModel;

fn main() {
    let phonemizer = PhonetisaurusModel::try_from(Path::new("model.fst")).unwrap();

    let result = phonemizer.phonemize_word("world").unwrap();
    assert_eq!(result.phonemes, "wˈɜɹld")
}
```

## License
`phonetisaurus-g2p-rs` is licensed under the [MIT License](LICENSE).
