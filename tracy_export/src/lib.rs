use std::{fs, io::prelude::*, path};

pub fn export(
    buf: &tracy_render::Buf,
    img_w: usize,
    img_h: usize,
    filename: &str,
) -> std::io::Result<()> {
    let folder_name = "render";
    fs::create_dir_all(folder_name)?;
    let mut file_path = path::PathBuf::from("render");
    file_path.push(format!("{}.ppm", filename));
    let mut file = fs::File::create(file_path)?;
    let header = format!("P3\n{} {}\n255\n", img_w, img_h);
    file.write_all(header.as_bytes())?;

    for pixel in buf {
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
