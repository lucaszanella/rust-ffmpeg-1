use std::ops::{Deref, DerefMut};
use std::vec::Vec;
use std::ptr;

//use super::{Audio, Check, Conceal, Opened, Subtitle, Video};
use super::super::context::Context as CodecContext;
use super::Context;
use super::super::super::util::cvec::CVec;
use ffi::*;
use {Dictionary, Discard, Error, Rational};

pub struct Parser<'a>(pub Context<'a>);

impl<'a> Parser<'a> {
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
    fn parse2(
        &self,
        buf: &[u8],
        pts: i64,
        dts: i64,
        pos: i64,
    ) -> Result<CVec, Error> {
        unsafe {// ptr::null(), ptr::null_mut()
            let poutbuf: *mut *mut u8;
            let poutbuf_size: *mut i32;
            match av_parser_parse2(self.as_mut_ptr(),
            self.av_codec_context.as_mut_ptr(),
            poutbuf,
            poutbuf_size,
            buf.as_ptr(),
            buf.len() as i32,
            pts,
            dts,
            pos
    ) {
                0 => {
                    //Ok(Vec::from_raw_parts(*poutbuf, *poutbuf_size, *poutbuf_size))
                    Ok(CVec::new(*poutbuf, *poutbuf_size as usize))
                },
                e => Err(Error::from(e)),
            }
        }
    }
}

impl<'a> Deref for Parser<'a> {
    type Target = Context<'a>;

    fn deref(&self) -> &<Self as Deref>::Target {
        &self.0
    }
}

impl<'a> DerefMut for Parser<'a> {
    fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
        &mut self.0
    }
}

impl<'a> AsRef<Context<'a>> for Parser<'a> {
    fn as_ref(&self) -> &Context<'a> {
        self
    }
}

impl<'a> AsMut<Context<'a>> for Parser<'a> {
    fn as_mut(&mut self) -> &mut Context<'a> {
        &mut self.0
    }
}
