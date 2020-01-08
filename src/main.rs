//! A basic output stream example, using an Output AudioUnit to generate a sine wave.

extern crate coreaudio;

use coreaudio::audio_unit::{AudioUnit, IOType, SampleFormat};
use coreaudio::audio_unit::render_callback::{self, data};

mod saw;
mod square;
mod sine;
mod tri;

fn main() -> Result<(), coreaudio::Error> {
    let frequency_hz = 440.;
    let volume = 0.15;
    let mut samples = tri::TriangleWaveGenerator::new(frequency_hz, volume);

    // Construct an Output audio unit that delivers audio to the default output device.
    let mut audio_unit = AudioUnit::new(IOType::DefaultOutput)?;

    let stream_format = audio_unit.output_stream_format()?;
    println!("{:#?}", &stream_format);

    // For this example, our sine wave expects `f32` data.
    assert!(SampleFormat::F32 == stream_format.sample_format);

    for i in 0..200 {
        println!("{:?} {:?}", i, samples.next().unwrap());
    }

    type Args = render_callback::Args<data::NonInterleaved<f32>>;
    audio_unit.set_render_callback(move |args| {
        let Args { num_frames, mut data, .. } = args;
        for i in 0..num_frames {
            let sample = samples.next().unwrap();
            for channel in data.channels_mut() {
                channel[i] = sample;
            }
        }
        Ok(())
    })?;
    audio_unit.start()?;


    std::thread::sleep(std::time::Duration::from_millis(3000));

    Ok(())
}
