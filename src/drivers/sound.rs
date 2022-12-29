use sdl2::audio::{AudioCallback, AudioSpecDesired};

pub struct SoundDriver {
    device: sdl2::audio::AudioDevice<SquareWave>,
    sound_on: bool,
} 
struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        // Generate a square wave
        for x in out.iter_mut() {
            *x = if self.phase <= 0.5 {
                self.volume
            } else {
                -self.volume
            };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}

impl SoundDriver {
    pub fn start(&mut self) {
        if !self.sound_on {
            self.sound_on = true;
            self.device.resume();    
        }
    }

    pub fn stop(&mut self) {
        if self.sound_on {
            self.device.pause();
            self.sound_on = false;
        }
    }

    pub fn new(context: &sdl2::Sdl) -> Self {
        let audio_subsystem = context.audio().unwrap();

        let desired_spec = AudioSpecDesired {
            freq: Some(44100),
            channels: Some(1),  // mono
            samples: None       // default sample size
        };

        let device = audio_subsystem.open_playback(None, &desired_spec, |spec| {
            // initialize the audio callback
            SquareWave {
                phase_inc: 440.0 / spec.freq as f32,
                phase: 0.0,
                volume: 0.25
            }
        }).unwrap();

        Self { 
            device,
            sound_on: false
        }
    }
}

