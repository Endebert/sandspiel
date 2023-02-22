extern crate core;

mod gui;

use crate::gui::Framework;
use log::{debug, error};
use pixels::{Pixels, SurfaceTexture};
use simulation::entities::cell_content::CellContent;
use simulation::entities::material::Material;
use simulation::sand_sim::{CellContentWrapper, Simulation};
use simulation::universe::{Position, Universe};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

const WIDTH: u32 = 128;
const HEIGHT: u32 = 128;

fn main() {
    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();

    let width_usize = WIDTH as usize;
    let height_usize = HEIGHT as usize;
    let mut sim = Simulation::new(width_usize, height_usize);
    let mut fill_area = vec![Material::Air; width_usize];
    fill_area[width_usize / 3] = Material::SandGenerator;
    fill_area[width_usize / 2] = Material::WaterGenerator;

    sim.fill(&fill_area);

    let mut current_tick = 0;

    let window = {
        let size = LogicalSize::new(WIDTH, HEIGHT);
        let scaled_size = LogicalSize::new(WIDTH * 4, HEIGHT * 4);
        WindowBuilder::new()
            .with_title("Sandspiel")
            .with_inner_size(scaled_size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture).unwrap()
    };

    // let window_size = window.inner_size();

    // truncation should be fine in this instance
    #[allow(clippy::cast_possible_truncation)]
    let scale_factor = window.scale_factor() as f32;

    let mut framework = Framework::new(&event_loop, WIDTH, HEIGHT, scale_factor, &pixels);

    let mut mouse_pos = (-1f32, -1f32);

    event_loop.run(move |event, _, control_flow| {
        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            if let Some(pos) = input.mouse() {
                mouse_pos = pos;
            }

            if input.mouse_pressed(0) | input.mouse_held(0) {
                match pixels.window_pos_to_pixel(mouse_pos) {
                    Ok((x, y)) => {
                        let content = CellContent::new(framework.gui.material.clone(), false, 0);
                        let position = Position::new(x, y);
                        sim.universe
                            .get_cell(&position)
                            .unwrap()
                            .lock()
                            .unwrap()
                            .clone_from(&content);
                    }
                    Err((x, y)) => {
                        debug!("mouse position outside of window!: {:?}:{:?}", x, y)
                    }
                }
            }

            // Update the scale factor
            if let Some(scale_factor) = input.scale_factor() {
                framework.scale_factor(scale_factor);
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height);
                framework.resize(size.width, size.height);
            }

            current_tick += 1;
            current_tick %= framework.gui.tick_interval;

            if current_tick == 0 {
                // Update internal state and request a redraw
                sim.tick();
            }
            window.request_redraw();
        }

        match event {
            Event::WindowEvent { event, .. } => {
                // Update egui inputs
                framework.handle_event(&event);
            }
            // Draw the current frame
            Event::RedrawRequested(_) => {
                // Draw the world
                draw(&sim.universe, pixels.get_frame_mut());

                // Prepare egui
                framework.prepare(&window);

                // Render everything together
                let render_result = pixels.render_with(|encoder, render_target, context| {
                    // Render the world texture
                    context.scaling_renderer.render(encoder, render_target);

                    // Render egui
                    framework.render(encoder, render_target, context);

                    Ok(())
                });

                // Basic error handling
                if render_result
                    .map_err(|e| error!("pixels.render() failed: {:?}", e))
                    .is_err()
                {
                    *control_flow = ControlFlow::Exit;
                }
            }
            _ => (),
        }
    });
}

fn draw(universe: &Universe<CellContentWrapper>, screen: &mut [u8]) {
    for (cell, pixel) in universe.area.iter().zip(screen.chunks_exact_mut(4)) {
        pixel.copy_from_slice(cell_to_color(&cell.lock().unwrap()));
    }
}

fn cell_to_color(cell: &CellContent) -> &[u8; 4] {
    match cell.material {
        Material::Sand => &SAND_COLOR,
        Material::SandGenerator => &SAND_GENERATOR_COLOR,
        Material::Water => &WATER_COLOR,
        Material::WaterGenerator => &WATER_GENERATOR_COLOR,
        Material::Air => &AIR_COLOR,
        Material::Fire => &FIRE_COLOR,
        Material::Smoke => &SMOKE_COLOR,
        Material::Vapor => &VAPOR_COLOR,
        Material::Wood => &WOOD_COLOR,
    }
}

const AIR_COLOR: [u8; 4] = [0xff, 0xff, 0xff, 0x00];
const WATER_COLOR: [u8; 4] = [0, 0, 0xff, 0xff];
const SAND_COLOR: [u8; 4] = [0xff, 0xff, 0, 0xff];
const WATER_GENERATOR_COLOR: [u8; 4] = [0, 0xff, 0xff, 0xff];
const SAND_GENERATOR_COLOR: [u8; 4] = [0xff, 0, 0xff, 0xff];
const FIRE_COLOR: [u8; 4] = [0xff, 0, 0, 0xff];
const SMOKE_COLOR: [u8; 4] = [0x7F, 0x7F, 0x7F, 0xff];
const VAPOR_COLOR: [u8; 4] = [0x7F, 0x7F, 0xFF, 0xff];
const WOOD_COLOR: [u8; 4] = [0xDE, 0xB8, 0x87, 0xff];
