# rust-morse-audio
Rust library to play morse code audio

## Usage

```rust
let morse_code = MorseCode::new()
    .frequency(400)
    .dot_duration(100)
    .amplify(0.7)
    .build();

morse_code.play("-.. -... ----- --.. ..-"); // DB0ZU
```

## References
Morse code https://en.wikipedia.org/wiki/Morse_code 