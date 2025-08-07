use std::{fs, io::prelude::*, path};
use tracy_render::Buf;

pub fn export(buf: Buf, filename: &str) -> std::io::Result<()> {
    let folder_name = "render";
    fs::create_dir_all(folder_name)?;
    let mut file_path = path::PathBuf::from("render");
    file_path.push(format!("{}.ppm", filename));
    let mut file = fs::File::create(file_path)?;
    let cols = buf.px_data.len() / buf.rows;
    let header = format!("P3\n{} {}\n255\n", cols, buf.rows);
    file.write_all(header.as_bytes())?;

    for pixel in buf.px_data {
        let line = format!("{} {} {}\n", pixel.r, pixel.g, pixel.b);
        file.write_all(line.as_bytes())?;
    }

    Ok(())
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         assert_eq!(1, 4);
//     }
// }
