use odio_lib::audio::trap_beat;
use odio_lib::host::get_audio_host;

fn main() {
    println!("Hello, world!");
    trap_beat(6);
    let host = get_audio_host();
}
