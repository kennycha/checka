use image::{ImageBuffer, Rgba, RgbaImage};
use std::path::Path;

// Generate robot head icon with the same design as menubar icon
fn generate_app_icon(size: u32, eye_color: [u8; 3]) -> RgbaImage {
    let mut img: RgbaImage = ImageBuffer::new(size, size);
    
    // Fill with transparent background
    for pixel in img.pixels_mut() {
        *pixel = Rgba([0, 0, 0, 0]);
    }
    
    let center_x = (size / 2) as i32;
    let center_y = (size / 2) as i32;
    
    // Scale factors based on size
    let scale = size as f32 / 32.0;
    
    // Draw robot head (outlined rounded rectangle) - scaled
    let head_width = (28.0 * scale) as i32;
    let head_height = (20.0 * scale) as i32;
    let corner_radius = (3.0 * scale) as i32;
    let outline_thickness = (2.0 * scale).max(1.0) as i32;
    
    // Position head slightly lower to make room for antenna
    let head_offset = (3.0 * scale) as i32;
    let head_left = center_x - head_width / 2;
    let head_right = center_x + head_width / 2;
    let head_top = center_y - head_height / 2 + head_offset;
    let head_bottom = center_y + head_height / 2 + head_offset;
    
    // Dark gray outline color
    let outline_color = Rgba([100, 100, 100, 255]);
    
    // Draw head outline
    for y in 0..size {
        for x in 0..size {
            let x_i = x as i32;
            let y_i = y as i32;
            
            let mut inside_outer = false;
            if x_i >= head_left + corner_radius && x_i <= head_right - corner_radius {
                if y_i >= head_top && y_i <= head_bottom {
                    inside_outer = true;
                }
            } else if y_i >= head_top + corner_radius && y_i <= head_bottom - corner_radius {
                if x_i >= head_left && x_i <= head_right {
                    inside_outer = true;
                }
            } else {
                // Check corners
                let corners = [
                    (head_left + corner_radius, head_top + corner_radius),
                    (head_right - corner_radius, head_top + corner_radius),
                    (head_left + corner_radius, head_bottom - corner_radius),
                    (head_right - corner_radius, head_bottom - corner_radius),
                ];
                
                for &(cx, cy) in &corners {
                    let dx = x_i - cx;
                    let dy = y_i - cy;
                    if dx * dx + dy * dy <= corner_radius * corner_radius {
                        inside_outer = true;
                        break;
                    }
                }
            }
            
            if inside_outer {
                let mut inside_inner = false;
                let inner_head_left = head_left + outline_thickness;
                let inner_head_right = head_right - outline_thickness;
                let inner_head_top = head_top + outline_thickness;
                let inner_head_bottom = head_bottom - outline_thickness;
                let inner_corner_radius = (corner_radius - outline_thickness).max(0);
                
                if x_i >= inner_head_left + inner_corner_radius && x_i <= inner_head_right - inner_corner_radius {
                    if y_i >= inner_head_top && y_i <= inner_head_bottom {
                        inside_inner = true;
                    }
                } else if y_i >= inner_head_top + inner_corner_radius && y_i <= inner_head_bottom - inner_corner_radius {
                    if x_i >= inner_head_left && x_i <= inner_head_right {
                        inside_inner = true;
                    }
                } else if inner_corner_radius > 0 {
                    let inner_corners = [
                        (inner_head_left + inner_corner_radius, inner_head_top + inner_corner_radius),
                        (inner_head_right - inner_corner_radius, inner_head_top + inner_corner_radius),
                        (inner_head_left + inner_corner_radius, inner_head_bottom - inner_corner_radius),
                        (inner_head_right - inner_corner_radius, inner_head_bottom - inner_corner_radius),
                    ];
                    
                    for &(cx, cy) in &inner_corners {
                        let dx = x_i - cx;
                        let dy = y_i - cy;
                        if dx * dx + dy * dy <= inner_corner_radius * inner_corner_radius {
                            inside_inner = true;
                            break;
                        }
                    }
                }
                
                if !inside_inner {
                    img.put_pixel(x, y, outline_color);
                }
            }
        }
    }
    
    // Draw antenna (small line on top)
    let antenna_x = center_x;
    let antenna_start_y = head_top - (2.0 * scale) as i32;
    let antenna_end_y = head_top - (6.0 * scale) as i32;
    let antenna_thickness = (1.0 * scale).max(1.0) as i32;
    
    for y in antenna_end_y..=antenna_start_y {
        for dx in -antenna_thickness/2..=antenna_thickness/2 {
            let x = antenna_x + dx;
            if x >= 0 && x < size as i32 && y >= 0 && y < size as i32 {
                img.put_pixel(x as u32, y as u32, outline_color);
            }
        }
    }
    
    // Draw robot eyes (asymmetric) - scaled
    let left_eye_radius = (3.0 * scale) as i32;
    let left_eye_x = center_x - (6.0 * scale) as i32;
    let left_eye_y = center_y + (1.0 * scale) as i32 + head_offset;
    
    let right_eye_radius = (4.0 * scale) as i32;
    let right_eye_x = center_x + (5.0 * scale) as i32;
    let right_eye_y = center_y + (2.0 * scale) as i32 + head_offset;
    
    let eye_outline_thickness = (1.0 * scale).max(1.0) as i32;
    
    // Draw left eye outline
    for y in 0..size {
        for x in 0..size {
            let dx = x as i32 - left_eye_x;
            let dy = y as i32 - left_eye_y;
            let distance_sq = dx * dx + dy * dy;
            let outer_radius_sq = (left_eye_radius + eye_outline_thickness) * (left_eye_radius + eye_outline_thickness);
            let inner_radius_sq = left_eye_radius * left_eye_radius;
            
            if distance_sq <= outer_radius_sq && distance_sq > inner_radius_sq {
                img.put_pixel(x, y, outline_color);
            }
        }
    }
    
    // Draw right eye outline
    for y in 0..size {
        for x in 0..size {
            let dx = x as i32 - right_eye_x;
            let dy = y as i32 - right_eye_y;
            let distance_sq = dx * dx + dy * dy;
            let outer_radius_sq = (right_eye_radius + eye_outline_thickness) * (right_eye_radius + eye_outline_thickness);
            let inner_radius_sq = right_eye_radius * right_eye_radius;
            
            if distance_sq <= outer_radius_sq && distance_sq > inner_radius_sq {
                img.put_pixel(x, y, outline_color);
            }
        }
    }
    
    // Draw left eye (filled center)
    let eye_rgba = Rgba([eye_color[0], eye_color[1], eye_color[2], 255]);
    for y in 0..size {
        for x in 0..size {
            let dx = x as i32 - left_eye_x;
            let dy = y as i32 - left_eye_y;
            let distance_sq = dx * dx + dy * dy;
            
            if distance_sq <= left_eye_radius * left_eye_radius {
                img.put_pixel(x, y, eye_rgba);
            }
        }
    }
    
    // Draw right eye (filled center)
    for y in 0..size {
        for x in 0..size {
            let dx = x as i32 - right_eye_x;
            let dy = y as i32 - right_eye_y;
            let distance_sq = dx * dx + dy * dy;
            
            if distance_sq <= right_eye_radius * right_eye_radius {
                img.put_pixel(x, y, eye_rgba);
            }
        }
    }
    
    img
}

fn main() {
    // Default eye color (gray)
    let eye_color = [107, 114, 128]; // #6b7280 gray
    
    // Generate different sizes
    let sizes_and_paths = vec![
        (32, "src-tauri/icons/32x32.png"),
        (128, "src-tauri/icons/128x128.png"),
        (256, "src-tauri/icons/128x128@2x.png"), // 2x version
        (1024, "src-tauri/icons/icon.png"), // For .icns generation
    ];
    
    for (size, path) in sizes_and_paths {
        println!("Generating {}x{} icon: {}", size, size, path);
        let img = generate_app_icon(size, eye_color);
        
        if let Err(e) = img.save(path) {
            eprintln!("Failed to save {}: {}", path, e);
        } else {
            println!("Saved: {}", path);
        }
    }
    
    println!("Icons updated! You may need to regenerate .icns/.ico files using appropriate tools.");
}