pub mod host {
    use platform::{ default_host, Host };
    use rodio::cpal::platform;


    pub fn get_audio_host() -> Host {
        match platform::host_from_id(platform::ALL_HOSTS[0]) {
            Ok(x) => x,
            Err(_) => default_host()
        }
    }
}

pub mod audio {
    use rodio::{source::Source, Decoder, OutputStream};
    use std::fs::File;
    use std::io::BufReader;
    use std::{thread, time};

    #[derive(Debug)]
    pub enum SoundEvent {
        Off,
        Play,
        Pause,
        Default,
    }

    fn _match_event(event: SoundEvent) -> &'static str {
        match event {
            SoundEvent::Off => "Off Event",
            SoundEvent::Play => "Play Event",
            SoundEvent::Pause => "Pause event",
            SoundEvent::Default => "Default event",
        }
    }

    pub fn trap_beat(duration: u64) {
        // Get handle to default output device
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        // Load a sound from a file, using a path relative to Cargo.toml
        let file = BufReader::new(File::open("input/trap-loop.mp3").unwrap());
        // Decode that sound file into a source
        let source = Decoder::new(file).unwrap();
        // Play the sound directly on the device
        stream_handle.play_raw(source.convert_samples()).unwrap();
        // The sound plays in a separate audio thread,
        // so we need to keep the main thread alive while it's playing.
        thread::sleep(time::Duration::from_secs(duration));
    }
}
