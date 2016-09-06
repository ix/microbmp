// µbmp - Tiny library for reading bitmap pixel data.

use std::io::prelude::*;
use std::fs::File;
use std::intrinsics::transmute;

// Pixel enumeratie type containing each BPP.
#[derive(Debug, Clone)]
pub enum Pixel {
  ARGB(u8, u8, u8, u8),
  RGB(u8, u8, u8)
}

// A basic (and incomplete) BITMAPV5HEADER.
#[derive(Debug, Clone)]
pub struct BitmapV5Header {
  pub size: u32,
  pub pix_width: i32,
  pub pix_height: i32,
  pub bpp: u16,
  pub colors: u32
}

// Containing Bitmap structure.
#[derive(Debug, Clone)]
pub struct Bitmap {
  pub data: Vec<u8>,
  pub size: u32,
  pub offset: u32,
  pub header: BitmapV5Header,
  pub pixels: Vec<Pixel>
}

impl Bitmap {
  pub fn new(file: &mut File) -> Bitmap {
    let mut buf: Vec<u8> = Vec::new();    
    file.read_to_end(&mut buf).expect("Failed to read the file.");

    let size: u32 = unsafe {
      let mut bytes: [u8; 4] = [0; 4];
      bytes.clone_from_slice(&buf[2..6]);
      transmute(bytes)
    };

    let offset: u32 = unsafe {
      let mut bytes: [u8; 4] = [0; 4];
      bytes.clone_from_slice(&buf[10..14]);
      transmute(bytes)
    };

    let header_size: u32 = unsafe {
      let mut bytes: [u8; 4] = [0; 4];
      bytes.clone_from_slice(&buf[14..18]);
      transmute(bytes)
    };

    let pix_width: i32 = unsafe {
      let mut bytes: [u8; 4] = [0; 4];
      bytes.clone_from_slice(&buf[18..22]);
      transmute(bytes)
    };

    let pix_height: i32 = unsafe {
      let mut bytes: [u8; 4] = [0; 4];
      bytes.clone_from_slice(&buf[22..26]);
      transmute(bytes)
    };
    
    let bpp: u16 = unsafe {
      let mut bytes: [u8; 2] = [0; 2];
      bytes.clone_from_slice(&buf[28..30]);
      transmute(bytes)
    };

    let colors: u32 = unsafe {
      let mut bytes: [u8; 4] = [0; 4];
      bytes.clone_from_slice(&buf[46..50]);
      transmute(bytes)
    };

    let pixel_data = match bpp {
      24 => {
        buf[offset as usize ..]
          .chunks(3)
          .map(|slice| {
            Pixel::RGB(slice[0], slice[1], slice[2])
          })
          .collect::<Vec<_>>()
      }

      32 => {
        buf[offset as usize ..]
          .chunks(4)
          .map(|slice| {
            Pixel::ARGB(slice[0], slice[1], slice[2], slice[3])
          })
          .collect::<Vec<_>>()
      }

      _ => { panic!("Unsupported bits-per-pixel!") }
    };
    
    Bitmap {
      data: buf,
      size: size,
      offset: offset,
      header: BitmapV5Header {
        size: header_size,
        pix_width: pix_width,
        pix_height: pix_height,
        bpp: bpp,
        colors: colors
      },
      pixels: pixel_data
    }
  }

  pub fn validity(&self) -> bool {
    if &self.data[0..1] != b"BM" {
      false
    }

    else {
      true
    }
  }
}
