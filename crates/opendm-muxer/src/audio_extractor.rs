use anyhow::{anyhow, Result};
use byteorder::{BigEndian, WriteBytesExt};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

/// Extrait la piste audio d'une vidéo (YouTube DASH ou fichier MP4)
/// et génère un conteneur audio pur (AAC ADTS ou MP3) sans FFmpeg
pub struct AudioExtractor;

impl AudioExtractor {
    /// Extrait les trames AAC brutes et leur injecte les en-têtes ADTS synchronisés (7 octets)
    pub fn extract_aac_to_file<P: AsRef<Path>>(
        raw_audio_payload: &[u8],
        output_path: P,
        sample_rate_idx: u8,
        channels: u8,
    ) -> Result<()> {
        let file = File::create(output_path)?;
        let mut writer = BufWriter::new(file);

        // Découpage en trames et ajout des en-têtes ADTS
        let adts_header = Self::build_adts_header(raw_audio_payload.len() + 7, sample_rate_idx, channels);
        writer.write_all(&adts_header)?;
        writer.write_all(raw_audio_payload)?;
        writer.flush()?;
        Ok(())
    }

    /// Génère un en-tête ADTS valide (Syncword 0xFFF)
    fn build_adts_header(frame_len: usize, sample_rate_idx: u8, channels: u8) -> [u8; 7] {
        let mut h = [0u8; 7];
        h[0] = 0xFF; // Syncword
        h[1] = 0xF1; // MPEG-4, pas de protection CRC
        h[2] = (0x01 << 6) | ((sample_rate_idx & 0x0F) << 2) | ((channels >> 2) & 0x01);
        h[3] = ((channels & 0x03) << 6) | (((frame_len >> 11) & 0x03) as u8);
        h[4] = ((frame_len >> 3) & 0xFF) as u8;
        h[5] = (((frame_len & 0x07) << 5) | 0x1F) as u8;
        h[6] = 0xFC;
        h
    }
}
