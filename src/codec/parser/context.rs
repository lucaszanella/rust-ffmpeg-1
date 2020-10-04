use std::ptr;
use std::rc::Rc;

use super::super::context::Context as CodecContext;
use super::parser::Parser;

//use super::{threading, Compliance, Debug, Flags, Id, Parameters};
use ffi::*;
use libc::c_int;
use media;
use {Codec, Error};

pub struct Context<'a> {
    pub av_codec_context: &'a mut CodecContext,
    ptr: *mut AVCodecParserContext,
    owner: Option<Rc<dyn Drop>>,
}

unsafe impl<'a> Send for Context<'a> {}

impl<'a> Context<'a> {
    pub unsafe fn wrap(
        av_codec_context: &'a mut CodecContext,
        ptr: *mut AVCodecParserContext,
        owner: Option<Rc<dyn Drop>>,
    ) -> Self {
        Context {
            av_codec_context,
            ptr,
            owner,
        }
    }

    pub unsafe fn as_ptr(&self) -> *const AVCodecParserContext {
        self.ptr as *const _
    }

    pub unsafe fn as_mut_ptr(&mut self) -> *mut AVCodecParserContext {
        self.ptr
    }
}

impl<'a> Context<'a> {
    pub fn new(av_codec_context: &'a mut CodecContext, codec_id: i32) -> Self {
        unsafe {
            Context {
                av_codec_context: av_codec_context,
                ptr: av_parser_init(codec_id),
                owner: None,
            }
        }
    }

    pub fn parser(self) -> Parser<'a> {
        Parser(self)
    }
}

impl<'a> Drop for Context<'a> {
    fn drop(&mut self) {
        unsafe {
            av_parser_close(self.ptr);
        }
    }
}
