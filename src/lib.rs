use rodio::cpal::traits::HostTrait;
use rodio::source::SineWave;
use rodio::{cpal, DeviceTrait, OutputStream, OutputStreamHandle};
use rodio::{Sink, Source};
use std::{thread, time};

#[derive(Debug, Clone, PartialEq)]
pub struct MorseCodeBuilder {
    frequency: Option<u32>,
    dot_duration: Option<u64>,
    amplify: Option<f32>,
    device_name: Option<String>,
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

    pub fn device_name(&mut self, name: &str) -> &mut Self {
        self.device_name = Some(name.to_owned());
        self
    }

    pub fn build(&mut self) -> MorseCode {
        MorseCode {
            frequency: self.frequency.unwrap_or(500),
            dot_duration: self.dot_duration.unwrap_or(80),
            amplify: self.amplify.unwrap_or(1.0),
            device_name: self.device_name.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MorseCode {
    frequency: u32,
    dot_duration: u64,
    amplify: f32,
    device_name: Option<String>,
}

impl MorseCode {
    pub fn new() -> MorseCodeBuilder {
        MorseCodeBuilder {
            frequency: None,
            dot_duration: None,
            amplify: None,
            device_name: None,
        }
    }

    pub fn play(&self, code: &str) {
        println!("play {}", code);

        let wave = SineWave::new(self.frequency as f32).amplify(self.amplify);
        let duration = time::Duration::from_millis(self.dot_duration);
        let dot_wave = wave.clone().take_duration(duration);
        let dash_wave = wave.clone().take_duration(duration * 3);
        // Default audio device
        let (mut _stream, mut stream_handle) = OutputStream::try_default().unwrap();
        // Named audio device
        if self.device_name.is_some() {
            (_stream, stream_handle) =
                select_output_stream(self.device_name.clone().unwrap().as_str());
        }
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

pub fn select_output_stream(device_name: &str) -> (OutputStream, OutputStreamHandle) {
    let host = cpal::default_host();
    // Fallback to default device if name not found
    let (mut _stream, mut stream_handle) = OutputStream::try_default().unwrap();

    if host.output_devices().is_ok() {
        let devices = host.output_devices().unwrap();
        for device in devices {
            let dev: rodio::Device = device.into();
            let dev_name = dev.name().unwrap();
            if dev_name == device_name {
                (_stream, stream_handle) = OutputStream::try_from_device(&dev).unwrap();
            }
        }
    }
    return (_stream, stream_handle);
}

#[cfg(test)]
mod tests {
    use super::*;
    use rodio::cpal::traits::HostTrait;
    use rodio::DeviceTrait;

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

    #[test]
    fn print_devices() {
        let host = cpal::default_host();

        if host.output_devices().is_ok() {
            let devices = host.output_devices().unwrap();

            for device in devices {
                // Print device
                let device: rodio::Device = device.into();
                println!("device {}", device.name().unwrap());
            }
        }
        assert_eq!(true, true);
    }

    #[test]
    fn select_audio_stream() {
        let (_stream, stream_handle) = select_output_stream("hw:CARD=sofhdadsp,DEV=0");

        assert_eq!(true, true);
    }
}
