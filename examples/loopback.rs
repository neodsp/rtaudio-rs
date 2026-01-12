use rtaudio::{
    Api, Buffers, DeviceParams, SampleFormat, StreamInfo, StreamOptions, StreamStatus,
    DEFAULT_BUFFER_FRAMES,
};

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
            Some(DeviceParams {
                num_channels: 2,
                ..Default::default()
            }),
            SampleFormat::Float32,
            None,
            DEFAULT_BUFFER_FRAMES,
            StreamOptions::default(),
            |error| eprintln!("{}", error),
        )
        .unwrap();
    dbg!(stream_handle.info());

    stream_handle
        .start(
            move |buffers: Buffers<'_>, _info: &StreamInfo, _status: StreamStatus| {
                if let Buffers::Float32 { output, input } = buffers {
                    // Copy the input to the output.
                    output.copy_from_slice(input);
                }
            },
        )
        .unwrap();

    // Wait 3 seconds before closing.
    std::thread::sleep(std::time::Duration::from_millis(3000));
}
