// µbmp - Tiny library for reading bitmap pixel data.

use std::io::prelude::*;
use std::fs::File;
use std::intrinsics::transmute;

// A basic (and incomplete) BITMAPV5HEADER.
#[derive(Debug, Clone)]
pub struct BitmapV5Header {
  pub size: u32,
  pub pix_width: i32
  pub pix_height: i32
  pub bpp: u16
  //pub size: u32
  //pub colors: u32
}

// Containing Bitmap structure.
#[derive(Debug, Clone)]
pub struct Bitmap {
  pub data: Vec<u8>,
  pub size: u32,
  pub offset: u32,
  pub header: BitmapV5Header
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
      let bytes: [u8; 2] = [0; 2];
      bytes.clone_from_slice(&buf[28..
    };
    
    Bitmap {
      data: buf,
      size: size,
      offset: offset,
      header: BitmapV5Header {
        size: header_size,
        pix_width: pix_width,
        pix_height: pix_height
      }
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
