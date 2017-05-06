use std::ffi::CString;
use std::env;

extern crate ffmpeg_sys;

fn get_codec_type(mut p_format_ctx: *mut ffmpeg_sys::AVFormatContext) {
    return;
}

fn process_vid(mut p_format_ctx: *mut ffmpeg_sys::AVFormatContext, x: String) {
    unsafe {
        let xcstr = CString::new(x).unwrap();

        //let ctx_ptr = &p_format_ctx

        if ffmpeg_sys::avformat_open_input(
                &mut p_format_ctx,
                xcstr.as_ptr(),
                std::ptr::null_mut(),
                std::ptr::null_mut()) != 0 {

            println!("Can't open file");
            return;
        }

        if ffmpeg_sys::avformat_find_stream_info(
                p_format_ctx,
                std::ptr::null_mut()) != 0 {

            println!("No stream info");
            return;
        }

        ffmpeg_sys::av_dump_format(p_format_ctx, 0, xcstr.as_ptr(), 0);

        let mut video_stream: i32 = -1;
        for i in 0..(*p_format_ctx).nb_streams {
            let stream = *((*p_format_ctx).streams.offset(i as isize));
            let ref codec_ctx: ffmpeg_sys::AVCodecContext = *((*stream).codec);

            if codec_ctx.codec_type == ffmpeg_sys::AVMEDIA_TYPE_VIDEO {

                video_stream = i as i32;
                break;
            }
        }

        if video_stream == -1 {
            println!("no video stream");
            return;
        }
    }
}

fn main() {
    unsafe {
        ffmpeg_sys::av_register_all();
        let mut p_format_ctx : *mut ffmpeg_sys::AVFormatContext = std::ptr::null_mut();

        let x = env::args().nth(1);

        match x {
            Some(x) => {
                process_vid(p_format_ctx, x);
            },

            None =>
                println!("No argument at 1"),
        }
    }
}
