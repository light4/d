use std::io::BufReader;

use anyhow::Result;
use rodio::{Decoder, OutputStream, Sink};
use tracing::info;

pub(crate) fn play<T>(buf: BufReader<T>) -> Result<()>
where
    T: std::io::Read + std::io::Seek + std::marker::Send + std::marker::Sync + 'static,
{
    // Get a output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&stream_handle).unwrap();
    info!("get output stream");
    let source = Decoder::new(buf).unwrap();
    info!("get decoded source");
    // Play the sound directly on the device
    sink.append(source);
    sink.sleep_until_end();
    Ok(())
}
