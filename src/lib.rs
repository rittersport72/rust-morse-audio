use rodio::source::SineWave;
use rodio::{Sink, Source};
use std::{thread, time};

#[derive(Debug, Clone, PartialEq)]
pub struct MorseCodeBuilder {
    frequency: Option<u32>,
    dot_duration: Option<u64>,
    amplify: Option<f32>,
}

impl MorseCodeBuilder {
    pub fn frequency(&mut self, frequency: u32) -> &mut Self {
        self.frequency = Some(frequency);
        self
    }

    pub fn dot_duration(&mut self, duration: u64) -> &mut Self {
        self.dot_duration = Some(duration);
        self
    }

    pub fn amplify(&mut self, amplify: f32) -> &mut Self {
        self.amplify = Some(amplify);
        self
    }

    pub fn build(&mut self) -> MorseCode {
        MorseCode {
            frequency: self.frequency.unwrap_or(500),
            dot_duration: self.dot_duration.unwrap_or(80),
            amplify: self.amplify.unwrap_or(1.0),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MorseCode {
    frequency: u32,
    dot_duration: u64,
    amplify: f32,
}

impl MorseCode {
    pub fn new() -> MorseCodeBuilder {
        MorseCodeBuilder {
            frequency: None,
            dot_duration: None,
            amplify: None,
        }
    }

    pub fn play(&self, code: &str) {
        println!("play {}", code);

        let wave = SineWave::new(self.frequency as f32).amplify(self.amplify);
        let duration = time::Duration::from_millis(self.dot_duration);
        let dot_wave = wave.clone().take_duration(duration);
        let dash_wave = wave.clone().take_duration(duration * 3);

        let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        for c in code.chars() {
            match c {
                '.' => {
                    sink.append(dot_wave.clone());
                    sink.sleep_until_end();
                    thread::sleep(duration);
                }
                '-' => {
                    sink.append(dash_wave.clone());
                    sink.sleep_until_end();
                    thread::sleep(duration);
                }
                ' ' => {
                    thread::sleep(duration * 3);
                }
                _ => {
                    thread::sleep(duration);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_play() {
        let morse_code = MorseCode::new()
            .frequency(400)
            .dot_duration(100)
            .amplify(0.7)
            .build();

        morse_code.play("-.. -... ----- --.. ..-"); // DB0ZU

        assert_eq!(true, true);
    }
}
