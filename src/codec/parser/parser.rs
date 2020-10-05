use std::ops::{Deref, DerefMut};
use std::ptr;
use std::vec::Vec;

//use super::{Audio, Check, Conceal, Opened, Subtitle, Video};
use super::super::super::packet::Packet;
use super::super::super::util::cvec::CVec;
use super::super::context::Context as CodecContext;
use super::Context;
use ffi::*;
use {Dictionary, Discard, Error, Rational};

pub struct Parser(pub Context);

impl Parser {
    /*
    int av_parser_parse2 	( 	AVCodecParserContext *  	s,
        AVCodecContext *  	avctx,
        uint8_t **  	poutbuf,
        int *  	poutbuf_size,
        const uint8_t *  	buf,
        int  	buf_size,
        int64_t  	pts,
        int64_t  	dts,
        int64_t  	pos
    )
    */
    fn parse2(&mut self, buf: &[u8], pts: i64, dts: i64, pos: i64) -> Result<CVec, Error> {
        unsafe {
            let mut poutbuf = self.packet.data_mut_ptr().unwrap();
            let mut poutbuf_size: i32 = 0;
            match av_parser_parse2(
                self.as_mut_ptr(),
                self.av_codec_context.borrow_mut().as_mut_ptr(),
                &mut poutbuf as *mut *mut u8,
                &mut poutbuf_size as *mut i32,
                buf.as_ptr(),
                buf.len() as i32,
                pts,
                dts,
                pos,
            ) {
                0 => Ok(CVec::new(poutbuf, poutbuf_size as usize)),
                e => Err(Error::from(e)),
            }
        }
    }
}

impl Deref for Parser {
    type Target = Context;

    fn deref(&self) -> &<Self as Deref>::Target {
        &self.0
    }
}

impl DerefMut for Parser {
    fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
        &mut self.0
    }
}

impl AsRef<Context> for Parser {
    fn as_ref(&self) -> &Context {
        self
    }
}

impl AsMut<Context> for Parser {
    fn as_mut(&mut self) -> &mut Context {
        &mut self.0
    }
}
