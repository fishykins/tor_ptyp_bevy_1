use super::FriedlanderWave;
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::time::Duration;

const TIMEOUT: u64 = 30;
const ENDTOLLERANCE: f32 = 0.00001;

// ========================================================================= //
// ============================== Structs ================================== //
// ========================================================================= //

/// A helpder struct for storing blast profiles
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlastProfile {
    pub delay: Duration,
    pub peak: f32,
    pub positive_phase_duration: Duration,
    pub curve: f32,
    pub bands: Vec<(BlastHarmonicBand, HarmonicGeneration)>,
    pub transient_clipping: u16,
}

/// The main wrapper for a blast sound.
#[derive(Clone)]
pub struct BlastWave {
    /// Harmonics to superimpose on the curve
    harmonics: Vec<BlastHarmonic>,
    /// Higher clipping = faster decay
    transient_clipping: u16,

    // Internal handlers
    sample: u64,
    last_curve_sample: f32,
    last_sine_sample: f32,
    friedlander: FriedlanderWave,
    length: Option<Duration>,
}

/// Represents a single harmonic frequency within a blast wave.
#[derive(Clone)]
struct BlastHarmonic {
    freq: f32,
    amplitude: f32,
    inversion: (f32, f32),
    offset: f32,
    diffraction: i32,
}

/// Types of blastwave generation.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum HarmonicGeneration {
    /// A bottom-up generation, where the wave is generated from a pre-defined set of harmonics.
    Cumulative,
    /// A pseudo-random generation, where the wave is generated from a random set of harmonics.
    Random,
}

/// A Frequency band for a blast wave.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlastHarmonicBand {
    pub frequency: f32,
    pub width: f32,
    pub amplitude: f32,
    pub weight: u32,
    pub diffraction: u32,
}

// ========================================================================= //

impl BlastProfile {
    pub fn from_file(name: &str) -> Self {
        let path_string = format!("{}.json", name);
        let mut file = match File::open(&path_string) {
            Err(why) => panic!("couldn't open {}: {}", path_string, why),
            Ok(file) => file,
        };

        let mut data = String::new();
        match file.read_to_string(&mut data) {
            Err(why) => panic!("couldn't read {}: {}", path_string, why),
            Ok(_) => (),
        }
        serde_json::from_str(&data).unwrap()
    }

    pub fn serialize(&self, name: String) -> Result<()> {
        let profile = serde_json::to_string(&self)?;

        let path_string = format!("{}.json", name);
        let path = Path::new(&path_string);
        let display = path.display();

        // Open a file in write-only mode, returns `io::Result<File>`
        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };

        match file.write_all(profile.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => println!("successfully wrote to {}", display),
        }
        Ok(())
    }
}

impl BlastHarmonic {
    fn cumulative_generation(i: usize, band: &BlastHarmonicBand, rng: &mut ThreadRng) -> Self {
        Self {
            freq: band.frequency
                * rng.gen_range(1f32..(1f32 + (1f32 + band.width) / i as f32))
                * i as f32,
            amplitude: 0.2f32.powf(1.0 / i as f32),
            inversion: if !rng.gen_bool(0.5) {
                (1.0, -1.0)
            } else {
                (-1.0, 1.0)
            },
            offset: rng.gen_range(-20.0..20.0),
            diffraction: (band.diffraction as f32 * rng.gen_range(0.75f32..1.25f32)) as i32,
        }
    }

    fn random_generation(band: &BlastHarmonicBand, rng: &mut ThreadRng) -> Self {
        Self {
            freq: band.frequency * rng.gen_range(1f32..band.width),
            amplitude: band.amplitude * rng.gen_range(0.1f32..1f32),
            inversion: if !rng.gen_bool(0.5) {
                (1.0, -1.0)
            } else {
                (-1.0, 1.0)
            },
            offset: rng.gen_range(-20.0..20.0),
            diffraction: (band.diffraction as f32 * rng.gen_range(0.75f32..1.25f32)) as i32,
        }
    }
}

impl BlastWave {
    pub fn new(profile: &BlastProfile) -> Self {
        let c = if profile.curve > 1.0 {
            profile.curve
        } else {
            1.01
        };
        let mut harmonics = Vec::new();
        let mut rng = rand::thread_rng();

        // Generate harmonics
        for (band, generation) in profile.bands.iter() {
            for i in 1..(1 + band.weight) {
                let h = match generation {
                    HarmonicGeneration::Random => BlastHarmonic::random_generation(band, &mut rng),
                    HarmonicGeneration::Cumulative => {
                        BlastHarmonic::cumulative_generation(i as usize, band, &mut rng)
                    }
                };
                harmonics.push(h);
            }
        }
        let mut me = Self {
            sample: 0,
            last_curve_sample: -100.0,
            last_sine_sample: -100.0,
            friedlander: FriedlanderWave::new(
                profile.delay.as_secs_f32(),
                profile.peak,
                profile.positive_phase_duration.as_secs_f32(),
                c,
            ),
            harmonics,
            transient_clipping: profile.transient_clipping,
            length: None,
        };

        // Aproximate the length by jumping in incriments of 1/100th of a second
        for t in 0..(TIMEOUT * 100) {
            let time = Duration::from_millis(t as u64);
            let synth_core = me.synthesize_core(time);
            let synth_sine = me.synthesize_harmonics(time);

            if synth_core.is_some() || synth_sine.is_some() {
                continue;
            }
            me.length = Some(time);
            break;
        }
        me.last_curve_sample = 1000.0;
        me.last_sine_sample = 1000.0;
        me.sample = 0;
        return me;
    }

    /// Calculates the generic curve of the gunshot, using the Friedlander equation.
    fn synthesize_core(&mut self, time: Duration) -> Option<f32> {
        // Primary pressure curve
        let a = self.friedlander.preasure(time.as_secs_f32());

        // Check the variance to see if the curve has stopped
        let variance = (self.last_curve_sample - a).abs();

        // Find the end point of the curve
        if variance <= 0.0001 && a > -0.001 || time > Duration::from_secs(TIMEOUT) {
            return None;
        }
        self.last_curve_sample = a;
        Some(a)
    }

    /// calculates the colouring at a given point along the main curve.
    fn synthesize_harmonics(&mut self, time: Duration) -> Option<f32> {
        let mut sine: f32 = 0.0;

        for h in self.harmonics.iter() {
            let amplitude =
                (1.0 + time.as_secs_f32()).powi(-(self.transient_clipping as i32)) * h.amplitude;
            let wave = ((time.as_secs_f32() + h.offset) * h.freq).sin();
            let notched = if wave >= 0.0 {
                h.inversion.1 * wave.powi(h.diffraction)
            } else {
                h.inversion.0 * wave.powi(h.diffraction)
            };
            sine += notched * amplitude;
        }

        // Check the variance to see if the curve has stopped
        let variance = (self.last_sine_sample - sine).abs();

        // Find the end point of the curve
        if (variance <= ENDTOLLERANCE && sine.abs() < ENDTOLLERANCE)
            || time > Duration::from_secs(TIMEOUT)
        {
            return None;
        }

        // Cache and return
        self.last_sine_sample = sine;
        Some(sine)
    }
}

impl Iterator for BlastWave {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<f32> {
        self.sample = self.sample.wrapping_add(1);
        let time = Duration::from_secs_f64((self.sample as f64) / (super::SAMPLERATE as f64));
        let synth_core = self.synthesize_core(time);
        let synth_sine = self.synthesize_harmonics(time);

        let mut result: f32 = 0.0;
        if synth_core.is_some() {
            result += synth_core.unwrap();
        }
        if synth_sine.is_some() {
            result += synth_sine.unwrap();
        }

        if synth_core.is_some() || synth_sine.is_some() {
            return Some(result);
        }
        // No results- end the loop and reset
        self.length = Some(Duration::from_millis((self.sample / 48) as u64));
        self.sample = 0;
        self.last_curve_sample = 1000.0;
        self.last_sine_sample = 1000.0;
        None
    }
}
