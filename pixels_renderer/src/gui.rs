use egui::{ClippedPrimitive, Context, TexturesDelta};
use egui_wgpu::renderer::{RenderPass, ScreenDescriptor};
use env_logger::TimestampPrecision::Seconds;
use pixels::{wgpu, PixelsContext};
use simulation::entities::material::Material;
use std::time::{Duration, SystemTime};
use winit::event_loop::EventLoopWindowTarget;
use winit::window::Window;

/// Manages all state required for rendering egui over `Pixels`.
pub(crate) struct Framework {
    // State for egui.
    egui_ctx: Context,
    egui_state: egui_winit::State,
    screen_descriptor: ScreenDescriptor,
    rpass: RenderPass,
    paint_jobs: Vec<ClippedPrimitive>,
    textures: TexturesDelta,

    // State for the GUI
    pub(crate) gui: Gui,
}

/// Example application state. A real application will need a lot more state than this.
pub struct Gui {
    pub material: Material,
    pub tick_interval: u8,
    pub frame_time: Duration,
    pub last_frame: SystemTime,
    pub failed_locks: usize,
}

impl Framework {
    /// Create egui.
    pub(crate) fn new<T>(
        event_loop: &EventLoopWindowTarget<T>,
        width: u32,
        height: u32,
        scale_factor: f32,
        pixels: &pixels::Pixels,
    ) -> Self {
        let max_texture_size = pixels.device().limits().max_texture_dimension_2d as usize;

        let egui_ctx = Context::default();
        let mut egui_state = egui_winit::State::new(event_loop);
        egui_state.set_max_texture_side(max_texture_size);
        egui_state.set_pixels_per_point(scale_factor);
        let screen_descriptor = ScreenDescriptor {
            size_in_pixels: [width, height],
            pixels_per_point: scale_factor,
        };
        let rpass = RenderPass::new(pixels.device(), pixels.render_texture_format(), 1);
        let textures = TexturesDelta::default();
        let gui = Gui {
            material: Material::Sand,
            tick_interval: 1,
            failed_locks: 0,
            frame_time: Duration::from_secs(0),
            last_frame: SystemTime::now(),
        };

        Self {
            egui_ctx,
            egui_state,
            screen_descriptor,
            rpass,
            paint_jobs: Vec::new(),
            textures,
            gui,
        }
    }

    /// Handle input events from the window manager.
    pub(crate) fn handle_event(&mut self, event: &winit::event::WindowEvent) {
        self.egui_state.on_event(&self.egui_ctx, event);
    }

    /// Resize egui.
    pub(crate) fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.screen_descriptor.size_in_pixels = [width, height];
        }
    }

    /// Update scaling factor.
    pub(crate) fn scale_factor(&mut self, scale_factor: f64) {
        self.screen_descriptor.pixels_per_point = scale_factor as f32;
    }

    /// Prepare egui.
    pub(crate) fn prepare(&mut self, window: &Window) {
        // Run the egui frame and create all paint jobs to prepare for rendering.
        let raw_input = self.egui_state.take_egui_input(window);
        let output = self.egui_ctx.run(raw_input, |egui_ctx| {
            // Draw the demo application.
            self.gui.ui(egui_ctx);
        });

        self.textures.append(output.textures_delta);
        self.egui_state
            .handle_platform_output(window, &self.egui_ctx, output.platform_output);
        self.paint_jobs = self.egui_ctx.tessellate(output.shapes);
    }

    /// Render egui.
    pub(crate) fn render(
        &mut self,
        encoder: &mut wgpu::CommandEncoder,
        render_target: &wgpu::TextureView,
        context: &PixelsContext,
    ) {
        // Upload all resources to the GPU.
        for (id, image_delta) in &self.textures.set {
            self.rpass
                .update_texture(&context.device, &context.queue, *id, image_delta);
        }
        self.rpass.update_buffers(
            &context.device,
            &context.queue,
            &self.paint_jobs,
            &self.screen_descriptor,
        );

        // Record all render passes.
        self.rpass.execute(
            encoder,
            render_target,
            &self.paint_jobs,
            &self.screen_descriptor,
            None,
        );

        // Cleanup
        let textures = std::mem::take(&mut self.textures);
        for id in &textures.free {
            self.rpass.free_texture(id);
        }
    }
}

impl Gui {
    /// Create the UI using egui.
    fn ui(&mut self, ctx: &Context) {
        egui::Window::new("Materials").show(ctx, |ui| {
            ui.vertical(|ui| {
                let current = &mut self.material;
                ui.radio_value(current, Material::Air, "Air");
                ui.radio_value(current, Material::Sand, "Sand");
                ui.radio_value(current, Material::Water, "Water");
                ui.radio_value(current, Material::Fire, "Fire");
                ui.radio_value(current, Material::Wood, "Wood");
                ui.radio_value(current, Material::Vapor, "Vapor");
                ui.radio_value(current, Material::Smoke, "Smoke");
                ui.radio_value(current, Material::SandGenerator, "Sand Generator");
                ui.radio_value(current, Material::WaterGenerator, "Water Generator");
            });
            ui.label("Tick Interval");
            ui.add(egui::Slider::new(&mut self.tick_interval, 1..=6));
            ui.label(format!("Failed locks: {}", self.failed_locks));

            let micros = self.frame_time.as_micros();

            ui.label(format!("Frame Time: {}µs", micros));
            ui.label(format!("FPS: {}", 1_000_000 / micros));
        });
    }
}
