use bevy::prelude::*;
use bevy_kira_audio::{
    AudioPlugin as KiraPlugin, AudioStream, AudioStreamPlugin, Frame, StreamedAudio,
};

/// Manages audio playback for the Torus client.
pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(KiraPlugin)
            .add_plugin(AudioStreamPlugin::<SineStream>::default());

        app.add_startup_system(start_stream);
    }
}

#[derive(Debug, Default)]
struct SineStream {
    t: f64,
    note: f64,
    frequency: f64,
}

impl AudioStream for SineStream {
    fn next(&mut self, _: f64) -> Frame {
        self.t += 2.0 * std::f64::consts::PI * self.note / self.frequency;
        Frame::from_mono(self.t.sin() as f32)
    }
}

fn start_stream(audio: Res<StreamedAudio<SineStream>>) {
    audio.stream(SineStream {
        t: 0.0,
        note: 440.0,
        frequency: 44_000.0,
    });
}
