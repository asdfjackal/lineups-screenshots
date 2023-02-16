use std::path::PathBuf;

use arboard::Clipboard;
use image::{DynamicImage, ImageBuffer, ImageFormat, RgbaImage};

fn main() {
    // Get map name from user input
    let mut map_name = String::new();
    println!("Enter map name:");
    std::io::stdin()
        .read_line(&mut map_name)
        .expect("Failed to read line");
    map_name = map_name.trim().to_string();

    // Get lineup type from user input
    let mut lineup_type = String::new();
    println!("Enter lineup type:");
    std::io::stdin()
        .read_line(&mut lineup_type)
        .expect("Failed to read line");
    lineup_type = lineup_type.trim().to_string();

    // Get current directory
    let current_dir = std::env::current_dir().unwrap();

    // Create folder for map and lineup type
    let map_dir = current_dir.join(map_name);
    let lineup_dir = map_dir.join(lineup_type);
    std::fs::create_dir_all(&lineup_dir).unwrap();

    // Get number of lineups from user input
    let mut num_lineups = String::new();
    println!("Enter number of lineups:");
    std::io::stdin()
        .read_line(&mut num_lineups)
        .expect("Failed to read line");
    let num_lineups: u32 = num_lineups.trim().parse().unwrap();

    // Get starting lineup from user input
    let mut start_lineup = String::new();
    println!("Enter starting lineup:");
    std::io::stdin()
        .read_line(&mut start_lineup)
        .expect("Failed to read line");
    let start_lineup: u32 = start_lineup.trim().parse().unwrap();

    // Initialize iterator
    let mut i = start_lineup - 1;

    // For each iteration, get four screenshots for the start, stand, angle, and target of the linup
    loop {
        println!("Collecting screenshots for lineup {}", i + 1);

        // Get start screenshot
        println!("Get starting position screenshot");
        std::io::stdin().read_line(&mut String::new()).unwrap();
        let start_path = lineup_dir.join(format!("start_{}.png", i));
        get_latest_screenshot(start_path);

        // Get stand screenshot
        println!("Get standing location screenshot");
        std::io::stdin().read_line(&mut String::new()).unwrap();
        let stand_path = lineup_dir.join(format!("stand_{}.png", i));
        get_latest_screenshot(stand_path);

        // Get angle screenshot
        println!("Get throw angle screenshot");
        std::io::stdin().read_line(&mut String::new()).unwrap();
        let angle_path = lineup_dir.join(format!("angle_{}.png", i));
        get_latest_screenshot(angle_path);

        // Get target screenshot
        println!("Get target screenshot");
        std::io::stdin().read_line(&mut String::new()).unwrap();
        let target_path = lineup_dir.join(format!("target_{}.png", i));
        get_latest_screenshot(target_path);

        // Increment iterator
        i += 1;

        // Check if we have reached the end
        if i == num_lineups {
            break;
        }
    }
}

fn get_latest_screenshot(target: PathBuf) {
    let mut clipboard = Clipboard::new().unwrap();
    let image = match clipboard.get_image() {
        Ok(img) => img,
        Err(e) => {
            eprintln!("error getting image: {}", e);
            return;
        }
    };

    let image: RgbaImage = ImageBuffer::from_raw(
        image.width.try_into().unwrap(),
        image.height.try_into().unwrap(),
        image.bytes.into_owned(),
    )
    .unwrap();
    let image = DynamicImage::ImageRgba8(image);

    // Save image to file
    image.save_with_format(target, ImageFormat::Png).unwrap();
}
