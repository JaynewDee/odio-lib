pub mod audio {
    use rodio::cpal::default_host;
    use rodio::cpal::{
        platform::Host,
        traits::{DeviceTrait, HostTrait},
    };
    use rodio::source::{SineWave, Source};
    use rodio::OutputStream;
    use rodio::Sink;
    use rodio::{Device, SupportedStreamConfig};
    use std::time::Duration;

    pub struct AudioHandle {
        host: Host,
        sink: Sink,
        frequency: f64,
    }

    impl Default for AudioHandle {
        fn default() -> Self {
            let host = default_host();
            let (_, stream_handle) = OutputStream::try_default().unwrap();

            let sink = Sink::try_new(&stream_handle).unwrap();

            AudioHandle {
                host,
                sink,
                frequency: 440.0,
            }
        }
    }

    #[derive(Debug)]
    pub enum Sound {
        SineWave,
        Source(String),
        Default,
    }

    impl AudioHandle {
        fn _play_sound(self, sound: Sound) {
            match sound {
                Sound::SineWave => println!("Sine Wave"),
                Sound::Source(_) => println!("From file source"),
                Sound::Default => println!("Default sound"),
            }
        }

        fn sine_wave(&self) {
            let source = SineWave::new(440.0)
                .take_duration(Duration::from_secs_f32(0.25))
                .amplify(0.20);

            self.sink.append(source);
            self.sink.sleep_until_end();
        }

        pub fn test(&self) {
            Self::sine_wave(self);
        }
    }
}
