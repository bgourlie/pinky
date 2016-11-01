#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(missing_copy_implementations)]

extern crate byteorder;

#[macro_use]
extern crate log;

#[macro_use]
extern crate emumisc;
extern crate mos6502;

#[macro_use]
extern crate bitflags;

#[cfg(test)]
#[macro_use]
extern crate nes_testsuite;

#[cfg(test)]
#[macro_use]
extern crate rp2c02_testsuite;

mod memory_map;
mod rp2c02_scheduler;
mod rp2c02;
mod virtual_apu;
mod virtual_nes;
mod rom;
mod mappers;
mod generic_mapper;
mod mapper_mmc1;
mod mapper_uxrom;
mod orphan;
mod dma;
mod filter;

#[cfg(test)]
mod testsuite;

pub use virtual_nes::{Interface, State, Context, Button, ControllerPort};
pub use rp2c02::{Framebuffer, Palette};
pub use rom::LoadError;
