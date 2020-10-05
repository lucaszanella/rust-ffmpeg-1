use std::ptr;
use std::rc::Rc;

use super::super::context::Context as CodecContext;
use super::super::packet::Packet;
use super::super::super::codec::Id;
use super::parser::Parser;

//use super::{threading, Compliance, Debug, Flags, Id, Parameters};
use ffi::*;
use libc::c_int;
use media;
use std::cell::RefCell;
use {Codec, Error};


pub struct Context {
    pub av_codec_context: RefCell<CodecContext>,
    ptr: *mut AVCodecParserContext,
    owner: Option<Rc<dyn Drop>>,
    pub packet: Packet,
}

unsafe impl Send for Context {}

impl Context {
    pub unsafe fn wrap(
        av_codec_context: RefCell<CodecContext>,
        ptr: *mut AVCodecParserContext,
        owner: Option<Rc<dyn Drop>>,
    ) -> Self {
        Context {
            av_codec_context,
            ptr,
            owner,
            packet: Packet::empty(),
        }
    }

    pub unsafe fn as_ptr(&self) -> *const AVCodecParserContext {
        self.ptr as *const _
    }

    pub unsafe fn as_mut_ptr(&mut self) -> *mut AVCodecParserContext {
        self.ptr
    }
}

impl Context {
    pub fn new(av_codec_context: RefCell<CodecContext>, codec_id: Id) -> Self {
        unsafe {
            Context {
                av_codec_context: av_codec_context,
                ptr: av_parser_init(Into::<AVCodecID>::into(codec_id) as i32),
                owner: None,
                packet: Packet::empty(),
            }
        }
    }

    pub fn parser(self) -> Parser {
        Parser(self)
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            av_parser_close(self.ptr);
        }
    }
}
