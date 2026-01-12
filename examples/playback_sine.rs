use rtaudio::{
    Api, Buffers, DeviceParams, SampleFormat, StreamInfo, StreamOptions, StreamStatus,
    DEFAULT_BUFFER_FRAMES,
};

const AMPLITUDE: f32 = 0.5;
const FREQ_HZ: f32 = 440.0;

fn main() {
    tracing::subscriber::set_global_default(
        tracing_subscriber::FmtSubscriber::builder()
            .with_max_level(tracing::Level::DEBUG)
            .finish(),
    )
    .unwrap();

    let host = rtaudio::Host::new(Api::Unspecified).unwrap();
    dbg!(host.api());

    let mut stream_handle = host
        .open_stream(
            Some(DeviceParams {
                num_channels: 2,
                ..Default::default()
            }),
            None,
            SampleFormat::Float32,
            None,
            DEFAULT_BUFFER_FRAMES,
            StreamOptions::default(),
            |error| eprintln!("{}", error),
        )
        .unwrap();
    dbg!(stream_handle.info());

    let mut phasor = 0.0;
    let phasor_inc = FREQ_HZ / stream_handle.info().sample_rate as f32;

    stream_handle
        .start(
            move |buffers: Buffers<'_>, _info: &StreamInfo, _status: StreamStatus| {
                if let Buffers::Float32 { output, input: _ } = buffers {
                    // By default, buffers are interleaved.
                    for frame in output.chunks_mut(2) {
                        // Generate a sine wave at 440 Hz at 50% volume.
                        let val = (phasor * std::f32::consts::TAU).sin() * AMPLITUDE;
                        phasor = (phasor + phasor_inc).fract();

                        frame[0] = val;
                        frame[1] = val;
                    }
                }
            },
        )
        .unwrap();

    // Wait 3 seconds before closing.
    std::thread::sleep(std::time::Duration::from_millis(3000));
}
