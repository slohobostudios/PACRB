use std::{
    error::Error,
    ffi::OsString,
    fs::{self, File},
    io::{self, BufRead, BufReader},
    path::Path,
};

use sfml::{graphics::Color, system::Vector2};
use tracing::error;

use crate::pallete_builder::color_grid::GRID_SIZE;

use super::{undo_redo::UndoRedoCell, ColorGrid};

const FILE_DIR: &str = "pacrb_files";

fn ensure_folder_exists() -> io::Result<()> {
    if Path::new(FILE_DIR).is_dir() {
        return Ok(());
    }

    fs::create_dir(FILE_DIR)
}

macro_rules! ensure_folder_exists {
    () => {
        if let Err(err) = ensure_folder_exists() {
            error!("{:#?}", err);
            return Default::default();
        }
    };
}

pub fn list_of_files_with_pacrb_extension() -> Vec<String> {
    ensure_folder_exists!();

    let Ok(files) = fs::read_dir(FILE_DIR) else {
        error!("Failed to read directory {}", FILE_DIR);
        return vec![];
    };

    files
        .filter_map(|dir_entry| {
            let Ok(dir_entry) = dir_entry else {
                return None;
            };

            let dir_entry = dir_entry.path();
            if dir_entry.extension() != Some(&OsString::from("pacrb")) {
                return None;
            }

            let Some(file_name) = dir_entry.file_name() else {
                return None;
            };

            let Some(file_name) = file_name.to_str() else {
                return None;
            };

            Some(file_name.to_owned())
        })
        .collect()
}

pub fn remove_pacrb_file(file_name: &str) {
    ensure_folder_exists!();
    let _ = fs::remove_file(format!("{}/{}", FILE_DIR, file_name));
}

////////////////////////////////////////////////////////////////////////////////////////////
/// Load and saving will follow this format. Every line will be like this:
///
/// (x_index,y_index):(r,g,b)
///
/// if x/y comboniation does not have an rgb value, mark it empty
/// my_file.pacrb:
/// 1 (25,25):(255,255,255)
/// 2 (26,25):(250,250,250)
////////////////////////////////////////////////////////////////////////////////////////////

// pub fn save_color_grid(color_grid: &ColorGrid, file_name: &str) {
//     ensure_folder_exists!();
//     todo!()
// }

pub fn load_color_grid(
    color_grid: &mut ColorGrid,
    file_name: &str,
    undo_redo: &mut UndoRedoCell,
) -> Result<(), Box<dyn Error>> {
    ensure_folder_exists()?;

    color_grid.0.iter_mut().for_each(|array| {
        array
            .iter_mut()
            .for_each(|color_cell| color_cell.borrow_mut().empty_the_cell(undo_redo))
    });

    let file = File::open(format!("{}/{}", FILE_DIR, file_name))?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let mut split = line.split(':');
        let (mut coordinates_string, mut color_string) = (
            split.next().ok_or("x/y index missing")?,
            split.next().ok_or("rgb value missing")?,
        );

        let (left_paranthesis_idx, right_paranthesis_idx) = (
            coordinates_string.find('(').ok_or(io::Error::new(
                io::ErrorKind::InvalidInput,
                "x/y tuple invalid",
            ))?,
            coordinates_string.find(')').ok_or(io::Error::new(
                io::ErrorKind::InvalidInput,
                "x/y tuple invalid",
            ))?,
        );
        coordinates_string = &coordinates_string[(left_paranthesis_idx + 1)..right_paranthesis_idx];
        let mut coordinates_split = coordinates_string.split(',');
        let coordinates = Vector2::new(
            coordinates_split
                .next()
                .ok_or("x/y value invalid")?
                .trim()
                .parse::<usize>()?,
            coordinates_split
                .next()
                .ok_or("x/y value invalid")?
                .trim()
                .parse::<usize>()?,
        );

        let (left_paranthesis_idx, right_paranthesis_idx) = (
            color_string.find('(').ok_or(io::Error::new(
                io::ErrorKind::InvalidInput,
                "rgb tuple invalid",
            ))?,
            color_string.find(')').ok_or(io::Error::new(
                io::ErrorKind::InvalidInput,
                "rgb tuple invalid",
            ))?,
        );
        color_string = &color_string[(left_paranthesis_idx + 1)..right_paranthesis_idx];
        let mut color_string_split = color_string.split(',');
        let color = Color::rgb(
            color_string_split
                .next()
                .ok_or("rgb value invalid")?
                .trim()
                .parse::<u8>()?,
            color_string_split
                .next()
                .ok_or("rgb value invalid")?
                .trim()
                .parse::<u8>()?,
            color_string_split
                .next()
                .ok_or("rgb value invalid")?
                .trim()
                .parse::<u8>()?,
        );

        if color_grid.is_idx_valid(coordinates) {
            color_grid[coordinates.x][coordinates.y]
                .borrow_mut()
                .fill_the_cell(undo_redo, color.into());
        } else {
            error!(
                "Coordinate values are not valid! {:#?}\nCoordinate values must be less than: {:#?}",
                coordinates, GRID_SIZE
            );
        }
    }

    Ok(())
}
