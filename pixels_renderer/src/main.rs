use log::{debug, error};
use pixels::{Pixels, SurfaceTexture};
use simulation::sand_sim::Simulation;
use simulation::universe::{CellContent, Material, Universe};
use winit::dpi::LogicalSize;
use winit::event::StartCause::Poll;
use winit::event::WindowEvent::{AxisMotion, CloseRequested, CursorMoved};
use winit::event::{Event, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

const WIDTH: u32 = 256;
const HEIGHT: u32 = 256;

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

    sim.universe.fill(&fill_area);

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

    // let mut now = SystemTime::now();

    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            // println!("draw at {}", now.elapsed().unwrap().as_millis());
            // now = SystemTime::now();
            draw(&sim.universe, pixels.get_frame_mut());
            if pixels
                .render()
                .map_err(|e| error!("pixels.render() failed: {:?}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
            // sim.tick();
            // window.request_redraw();
        }

        if input.update(&event) {
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height);
            }

            sim.tick();
            window.request_redraw();
        }
    });
}

fn draw(universe: &Universe, screen: &mut [u8]) {
    for (cell, pixel) in universe.area.iter().zip(screen.chunks_exact_mut(4)) {
        pixel.copy_from_slice(cell_to_color(cell));
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
const SMOKE_COLOR: [u8; 4] = [0x77, 0x77, 0x77, 0x77];
const VAPOR_COLOR: [u8; 4] = [0, 0, 0xff, 0x77];
const WOOD_COLOR: [u8; 4] = [0xDE, 0xB8, 0x87, 0xff];
