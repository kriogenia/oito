use sdl2::audio::{AudioSpecDesired, AudioSpecWAV, AudioCallback};

pub const SOUND_SPEC: AudioSpecDesired = AudioSpecDesired {
    freq: Some(44100),
    channels: Some(1), // mono
    samples: None,     // default sample size
};

pub struct Beep {
    bytes: Vec<u8>,
    position: usize
}

impl Beep {
    /// Builds a beep sound to perform from a WAV path
    pub fn new(path: String) -> Self {
		let wav = AudioSpecWAV::load_wav(path).expect("invalid beep file");
		
		Self {
            bytes: wav.buffer().to_vec(),
			position: 0
        }
    }
}

impl AudioCallback for Beep {
    type Channel = u8;

    fn callback(&mut self, data: &mut [u8]) {
        let (start, end) = (self.position, self.position + data.len());
        self.position += data.len();

        let audio_data = &self.bytes[start..end];
        for (src, dst) in audio_data.iter().zip(data.iter_mut()) {
            *dst = *src;
        }
    }
}