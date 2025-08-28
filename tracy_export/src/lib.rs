use std::{fs, io::prelude::*, path};
use tracy_macros::Color;
use tracy_render::FrameBuf;

pub fn export(buf: Vec<FrameBuf>) -> std::io::Result<()> {
    let folder_name = "render";
    fs::create_dir_all(folder_name)?;

    for (frame, frame_buf) in buf.into_iter().enumerate() {
        let mut file_path = path::PathBuf::from("render");
        file_path.push(format!("frame_{}.ppm", frame));
        let mut file = fs::File::create(file_path)?;

        let cols = frame_buf.frame.len() / frame_buf.rows;
        let header = format!("P3\n{} {}\n255\n", cols, frame_buf.rows);
        file.write_all(header.as_bytes())?;

        for px in frame_buf.frame {
            let px = px.to_u8();
            let line = format!("{} {} {}\n", px.r, px.g, px.b);
            file.write_all(line.as_bytes())?;
        }
    }

    Ok(())
}
