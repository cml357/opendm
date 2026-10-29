use byteorder::{BigEndian, WriteBytesExt};
use std::io::Write;

pub struct Mp4BoxWriter;

impl Mp4BoxWriter {
    pub fn write_box(fourcc: &[u8; 4], payload: &[u8]) -> Vec<u8> {
        let size = 8 + payload.len() as u32;
        let mut buf = Vec::with_capacity(size as usize);
        buf.write_u32::<BigEndian>(size).unwrap();
        buf.write_all(fourcc).unwrap();
        buf.write_all(payload).unwrap();
        buf
    }

    pub fn build_ftyp() -> Vec<u8> {
        let mut p = Vec::new();
        p.write_all(b"isom").unwrap();
        p.write_u32::<BigEndian>(0x00000200).unwrap();
        p.write_all(b"isomiso2avc1mp41").unwrap();
        Self::write_box(b"ftyp", &p)
    }

    pub fn build_mvhd(timescale: u32, duration_units: u32) -> Vec<u8> {
        let mut p = Vec::new();
        p.write_u8(0).unwrap(); // Version
        p.write_all(&[0, 0, 0]).unwrap(); // Flags
        p.write_u32::<BigEndian>(0).unwrap();
        p.write_u32::<BigEndian>(0).unwrap();
        p.write_u32::<BigEndian>(timescale).unwrap();
        p.write_u32::<BigEndian>(duration_units).unwrap();
        p.write_u32::<BigEndian>(0x00010000).unwrap(); // Rate 1.0
        p.write_u16::<BigEndian>(0x0100).unwrap(); // Volume 1.0
        p.write_all(&[0; 10]).unwrap();
        // Matrice unitaire d'affichage
        for val in [0x00010000, 0, 0, 0, 0x00010000, 0, 0, 0, 0x40000000] {
            p.write_u32::<BigEndian>(val).unwrap();
        }
        p.write_all(&[0; 24]).unwrap();
        p.write_u32::<BigEndian>(3).unwrap(); // Next track id
        Self::write_box(b"mvhd", &p)
    }
}
