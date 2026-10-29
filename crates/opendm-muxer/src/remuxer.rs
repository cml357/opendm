use crate::mp4_box::Mp4BoxWriter;
use anyhow::Result;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

pub struct VideoStreamConfig {
    pub width: u32,
    pub height: u32,
    pub fps: u32,
}

pub struct AudioStreamConfig {
    pub sample_rate: u32,
    pub channels: u16,
}

pub struct RemuxEngine;

impl RemuxEngine {
    pub fn mux_avc_aac<P: AsRef<Path>>(
        output_file: P,
        video_data: &[u8],
        audio_data: &[u8],
        _v_cfg: &VideoStreamConfig,
        _a_cfg: &AudioStreamConfig,
    ) -> Result<()> {
        let f = File::create(output_file)?;
        let mut writer = BufWriter::new(f);

        // 1. ftyp box
        writer.write_all(&Mp4BoxWriter::build_ftyp())?;

        // 2. mdat box contenant les données brutes
        let mut mdat_payload = Vec::with_capacity(video_data.len() + audio_data.len());
        mdat_payload.extend_from_slice(video_data);
        mdat_payload.extend_from_slice(audio_data);
        writer.write_all(&Mp4BoxWriter::write_box(b"mdat", &mdat_payload))?;

        // 3. moov metadata
        let mvhd = Mp4BoxWriter::build_mvhd(90000, 90000 * 10);
        writer.write_all(&Mp4BoxWriter::write_box(b"moov", &mvhd))?;

        writer.flush()?;
        Ok(())
    }
}
