use std::ffi::CString;
use std::env;

extern crate ffmpeg_sys;

fn get_codec_type(mut p_format_ctx: *mut ffmpeg_sys::AVFormatContext) {
    return;
}

// This isn't doing the desired side effects... hmm.
// Something related to mutability -- will see what this is all about
fn ffmpeg_open_file(mut p_format_ctx: *mut ffmpeg_sys::AVFormatContext, filename: &CString) -> bool {
    let f = String::from_utf8(filename.as_bytes().to_vec());
    match f {
        Ok(s) =>
            println!("attempting to open file {}", s),
        Err(e) => ()
    };

    return unsafe {
        if ffmpeg_sys::avformat_open_input(
                &mut p_format_ctx,
                filename.as_ptr(),
                std::ptr::null_mut(),
                std::ptr::null_mut()) != 0 {

            false
        } else {
            true
        }
    };
}

fn process_vid(mut p_format_ctx: *mut ffmpeg_sys::AVFormatContext, x: String) {
    unsafe {
        let xcstr = CString::new(x).unwrap();
        let openresult = ffmpeg_open_file(p_format_ctx, &xcstr);

        println!("open result: {}", openresult);

        if !openresult {
            println!("Can't open file");
            return;
        }

        // Unsafe
        if ffmpeg_sys::avformat_find_stream_info(
                p_format_ctx,
                std::ptr::null_mut()) != 0 {

            println!("No stream info");
            return;
        }

        // Unsafe function
        ffmpeg_sys::av_dump_format(p_format_ctx, 0, xcstr.as_ptr(), 0);

        let mut video_stream: i32 = -1;
        for i in 0..(*p_format_ctx).nb_streams {

            // Dereference of raw pointer
            // Invocation of unsafe method
            let stream = *((*p_format_ctx).streams.offset(i as isize));

            //Dereference of raw pointer (2)
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

fn get_ffmpeg_context() -> *mut ffmpeg_sys::AVFormatContext {
    return unsafe {
        ffmpeg_sys::av_register_all();
        std::ptr::null_mut()
    };
}

fn main() {
    let mut p_format_ctx : *mut ffmpeg_sys::AVFormatContext = get_ffmpeg_context();

    let x = env::args().nth(1);

    match x {
        Some(x) => {
            process_vid(p_format_ctx, x);
        },

        None =>
            println!("No argument at 1"),
    }
}
