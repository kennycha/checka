use tauri::{
    image::Image,
    tray::{MouseButtonState, TrayIcon, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager,
};
use tauri_nspanel::ManagerExt;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering};
use image::{ImageBuffer, Rgba, RgbaImage, ImageEncoder};

use crate::fns::position_menubar_panel;
use crate::agent_manager::AgentManager;

pub fn create(app_handle: &AppHandle) -> tauri::Result<TrayIcon> {
    let initial_icon = get_tray_icon_for_count(0, 0)?;

    let tray = TrayIconBuilder::with_id("tray")
        .icon(initial_icon)
        .icon_as_template(true)
        .on_tray_icon_event(|tray, event| {
            let app_handle = tray.app_handle();

            if let TrayIconEvent::Click { button_state, .. } = event {
                if button_state == MouseButtonState::Up {
                    let panel = app_handle.get_webview_panel("main").unwrap();

                    if panel.is_visible() {
                        panel.order_out(None);
                        return;
                    }

                    position_menubar_panel(app_handle, 0.0);

                    panel.show();
                }
            }
        })
        .build(app_handle)?;

    // Store tray reference for updates
    app_handle.manage(Arc::new(Mutex::new(tray.clone())));
    
    // Store animation frame counter
    app_handle.manage(Arc::new(AtomicUsize::new(0)));

    // Start background task to update tray icon
    start_tray_updater(app_handle.clone());

    Ok(tray)
}

fn get_tray_icon_for_count(processing_count: usize, waiting_count: usize) -> tauri::Result<Image<'static>> {
    if processing_count == 0 && waiting_count == 0 {
        // All off - use gray eyes
        generate_robot_head_icon([107, 114, 128], processing_count + waiting_count) // #6b7280 gray
    } else if processing_count > 0 {
        // Has processing agents - use softer green eyes
        generate_robot_head_icon([52, 168, 83], processing_count + waiting_count) // #34a853 softer green
    } else {
        // Only waiting agents - use softer yellow eyes
        generate_robot_head_icon([251, 188, 4], processing_count + waiting_count) // #fbbc04 softer yellow
    }
}


fn generate_robot_head_icon(eye_color: [u8; 3], count: usize) -> tauri::Result<Image<'static>> {
    // Generate robot head icon with colored eyes
    let size = 32u32;
    let mut img: RgbaImage = ImageBuffer::new(size, size);
    
    // Fill with transparent background
    for pixel in img.pixels_mut() {
        *pixel = Rgba([0, 0, 0, 0]);
    }
    
    let center_x = (size / 2) as i32;
    let center_y = (size / 2) as i32;
    
    // Draw robot head (outlined rounded rectangle) - reduced size to make room for antenna
    let head_width = 28;  // Reduced size for antenna space
    let head_height = 20; // Reduced size for antenna space
    let corner_radius = 3; // Proportionally smaller corner radius
    let outline_thickness = 2;
    
    // Position head slightly lower to make room for antenna
    let head_left = center_x - head_width / 2;
    let head_right = center_x + head_width / 2;
    let head_top = center_y - head_height / 2 + 3; // Moved down 3 pixels for antenna
    let head_bottom = center_y + head_height / 2 + 3;
    
    // Dark gray outline color
    let outline_color = Rgba([100, 100, 100, 255]);
    
    for y in 0..size {
        for x in 0..size {
            let x_i = x as i32;
            let y_i = y as i32;
            
            // Check if point is on the outline of rounded rectangle
            let mut on_outline = false;
            
            // Check if point is within outer rounded rectangle
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
                // Check outer corners
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
            
            // Check if point is within inner rounded rectangle (for hollow effect)
            let mut inside_inner = false;
            let inner_left = head_left + outline_thickness;
            let inner_right = head_right - outline_thickness;
            let inner_top = head_top + outline_thickness;
            let inner_bottom = head_bottom - outline_thickness;
            let inner_corner_radius = corner_radius - outline_thickness;
            
            if inner_corner_radius > 0 {
                if x_i >= inner_left + inner_corner_radius && x_i <= inner_right - inner_corner_radius {
                    if y_i >= inner_top && y_i <= inner_bottom {
                        inside_inner = true;
                    }
                } else if y_i >= inner_top + inner_corner_radius && y_i <= inner_bottom - inner_corner_radius {
                    if x_i >= inner_left && x_i <= inner_right {
                        inside_inner = true;
                    }
                } else {
                    // Check inner corners
                    let inner_corners = [
                        (inner_left + inner_corner_radius, inner_top + inner_corner_radius),
                        (inner_right - inner_corner_radius, inner_top + inner_corner_radius),
                        (inner_left + inner_corner_radius, inner_bottom - inner_corner_radius),
                        (inner_right - inner_corner_radius, inner_bottom - inner_corner_radius),
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
            }
            
            // Draw outline (inside outer but not inside inner)
            if inside_outer && !inside_inner {
                on_outline = true;
            }
            
            if on_outline {
                img.put_pixel(x, y, outline_color);
            }
        }
    }
    
    // Draw robot eyes (asymmetric like the reference image) - proportionally adjusted
    // Left eye - smaller and slightly higher
    let left_eye_radius = 3;  // Proportionally smaller
    let left_eye_x = center_x - 6;  // Adjusted for smaller head
    let left_eye_y = center_y + 1;  // Adjusted for moved head position
    
    // Right eye - larger and slightly lower
    let right_eye_radius = 4;  // Proportionally smaller
    let right_eye_x = center_x + 5;  // Adjusted for smaller head
    let right_eye_y = center_y + 2;  // Adjusted for moved head position
    
    let eye_outline_thickness = 1;  // Thinner outline for smaller eyes
    
    // Draw left eye outline first
    for y in 0..size {
        for x in 0..size {
            let dx = x as i32 - left_eye_x;
            let dy = y as i32 - left_eye_y;
            let distance_sq = dx * dx + dy * dy;
            let outer_radius_sq = (left_eye_radius + eye_outline_thickness) * (left_eye_radius + eye_outline_thickness);
            let inner_radius_sq = left_eye_radius * left_eye_radius;
            
            // Draw outline (between outer and inner radius)
            if distance_sq <= outer_radius_sq && distance_sq > inner_radius_sq {
                img.put_pixel(x, y, outline_color);
            }
        }
    }
    
    // Draw right eye outline first
    for y in 0..size {
        for x in 0..size {
            let dx = x as i32 - right_eye_x;
            let dy = y as i32 - right_eye_y;
            let distance_sq = dx * dx + dy * dy;
            let outer_radius_sq = (right_eye_radius + eye_outline_thickness) * (right_eye_radius + eye_outline_thickness);
            let inner_radius_sq = right_eye_radius * right_eye_radius;
            
            // Draw outline (between outer and inner radius)
            if distance_sq <= outer_radius_sq && distance_sq > inner_radius_sq {
                img.put_pixel(x, y, outline_color);
            }
        }
    }
    
    // Draw left eye (smaller) - filled center
    for y in 0..size {
        for x in 0..size {
            let dx = x as i32 - left_eye_x;
            let dy = y as i32 - left_eye_y;
            let distance_sq = dx * dx + dy * dy;
            
            if distance_sq <= left_eye_radius * left_eye_radius {
                let eye_rgba = Rgba([eye_color[0], eye_color[1], eye_color[2], 255]);
                img.put_pixel(x, y, eye_rgba);
            }
        }
    }
    
    // Draw right eye (larger) - filled center
    for y in 0..size {
        for x in 0..size {
            let dx = x as i32 - right_eye_x;
            let dy = y as i32 - right_eye_y;
            let distance_sq = dx * dx + dy * dy;
            
            if distance_sq <= right_eye_radius * right_eye_radius {
                let eye_rgba = Rgba([eye_color[0], eye_color[1], eye_color[2], 255]);
                img.put_pixel(x, y, eye_rgba);
            }
        }
    }
    
    // Draw robot antennas
    let antenna_color = outline_color; // Same color as head outline
    
    // Left antenna
    let left_antenna_x = center_x - 4;
    let left_antenna_base_y = head_top - 1;
    let left_antenna_top_y = head_top - 6;
    
    // Draw left antenna line
    for y in left_antenna_top_y..=left_antenna_base_y {
        if y >= 0 && y < size as i32 {
            img.put_pixel(left_antenna_x as u32, y as u32, antenna_color);
        }
    }
    
    // Draw left antenna ball (small circle)
    let antenna_ball_radius = 1;
    for dy in -antenna_ball_radius..=antenna_ball_radius {
        for dx in -antenna_ball_radius..=antenna_ball_radius {
            let ball_x = left_antenna_x + dx;
            let ball_y = left_antenna_top_y + dy;
            if dx * dx + dy * dy <= antenna_ball_radius * antenna_ball_radius {
                if ball_x >= 0 && ball_x < size as i32 && ball_y >= 0 && ball_y < size as i32 {
                    img.put_pixel(ball_x as u32, ball_y as u32, antenna_color);
                }
            }
        }
    }
    
    // Right antenna
    let right_antenna_x = center_x + 4;
    let right_antenna_base_y = head_top - 1;
    let right_antenna_top_y = head_top - 6;
    
    // Draw right antenna line
    for y in right_antenna_top_y..=right_antenna_base_y {
        if y >= 0 && y < size as i32 {
            img.put_pixel(right_antenna_x as u32, y as u32, antenna_color);
        }
    }
    
    // Draw right antenna ball (small circle)
    for dy in -antenna_ball_radius..=antenna_ball_radius {
        for dx in -antenna_ball_radius..=antenna_ball_radius {
            let ball_x = right_antenna_x + dx;
            let ball_y = right_antenna_top_y + dy;
            if dx * dx + dy * dy <= antenna_ball_radius * antenna_ball_radius {
                if ball_x >= 0 && ball_x < size as i32 && ball_y >= 0 && ball_y < size as i32 {
                    img.put_pixel(ball_x as u32, ball_y as u32, antenna_color);
                }
            }
        }
    }
    
    // Add small indicators for count (tiny dots on robot head)
    if count > 0 {
        let display_count = if count > 9 { 9 } else { count };
        
        // Place small dots on the top of the robot head
        for i in 0..display_count {
            let dot_x = head_left + 3 + (i * 2) as i32;
            let dot_y = head_top + 2;
            
            if dot_x < head_right - 2 && dot_y < head_bottom {
                // Small white indicator dot
                img.put_pixel(dot_x as u32, dot_y as u32, Rgba([255, 255, 255, 255]));
            }
        }
    }
    
    // Convert to PNG bytes
    let mut png_bytes = Vec::new();
    {
        let mut cursor = std::io::Cursor::new(&mut png_bytes);
        if image::codecs::png::PngEncoder::new(&mut cursor)
            .write_image(&img, size, size, image::ColorType::Rgba8)
            .is_ok()
        {
            Image::from_bytes(&png_bytes)
        } else {
            // Fallback to default icon if generation fails
            Image::from_bytes(include_bytes!("../icons/tray.png"))
        }
    }
}

fn get_animated_robot_head_icon(frame: usize, processing_count: usize, waiting_count: usize) -> tauri::Result<Image<'static>> {
    // Generate robot head (same as static version - no animation on eyes)
    // Just use the regular robot head function and ignore frame parameter
    let _ = frame; // Suppress unused parameter warning
    
    // Determine eye color based on priority: processing > waiting
    let eye_color = if processing_count > 0 {
        [52, 168, 83] // Softer green for processing
    } else {
        [251, 188, 4] // Softer yellow for waiting
    };
    
    // Use the same robot head generation as static version
    generate_robot_head_icon(eye_color, processing_count + waiting_count)
}

fn start_tray_updater(app_handle: AppHandle) {
    // Use std::thread instead of tokio::spawn to avoid runtime issues
    std::thread::spawn(move || {
        loop {
            if let Some(tray_ref) = app_handle.try_state::<Arc<Mutex<TrayIcon>>>() {
                if let Some(frame_counter) = app_handle.try_state::<Arc<AtomicUsize>>() {
                    let manager = AgentManager::new();
                    let processing_count = manager.get_processing_count();
                    let waiting_count = manager.get_waiting_count();
                    
                    let new_icon = if processing_count > 0 || waiting_count > 0 {
                        // Active agents - use animated robot head
                        let current_frame = frame_counter.fetch_add(1, Ordering::Relaxed);
                        get_animated_robot_head_icon(current_frame, processing_count, waiting_count)
                    } else {
                        // All off - use static robot head
                        frame_counter.store(0, Ordering::Relaxed); // Reset animation
                        get_tray_icon_for_count(processing_count, waiting_count)
                    };
                    
                    if let Ok(icon) = new_icon {
                        if let Ok(tray) = tray_ref.lock() {
                            let _ = tray.set_icon(Some(icon));
                            
                            // Update tooltip to show status
                            let tooltip = match (processing_count, waiting_count) {
                                (0, 0) => "AI Agents - All off".to_string(),
                                (p, 0) => format!("AI Agents - {} processing", p),
                                (0, w) => format!("AI Agents - {} waiting", w),
                                (p, w) => format!("AI Agents - {} processing, {} waiting", p, w),
                            };
                            let _ = tray.set_tooltip(Some(&tooltip));
                        }
                    }
                }
            }
            
            // Different sleep times based on whether we're animating
            let manager = AgentManager::new();
            let has_active_agents = manager.get_processing_count() > 0 || manager.get_waiting_count() > 0;
            let sleep_duration = if has_active_agents {
                // Faster updates for animation when active
                std::time::Duration::from_millis(500)
            } else {
                // Slower updates when all off
                std::time::Duration::from_secs(3)
            };
            
            std::thread::sleep(sleep_duration);
        }
    });
}

