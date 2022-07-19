use glam::{DVec2, DVec3, UVec2};
use instant::Instant;
use winit::event::{ElementState, VirtualKeyCode, WindowEvent};

use super::Body;

pub struct State<'a> {
    pub mouse_pos: DVec2,
    pub window_size: UVec2,
    pub last_frame: Option<Instant>,
    pub bodies: Vec<Body>,
    pub wireframe: bool,
    pub paused: bool,
    pub bg_color: DVec3,
    pub texture_key: &'a str,
}

impl<'a> Default for State<'a> {
    fn default() -> Self {
        Self {
            mouse_pos: DVec2::default(),
            window_size: UVec2::default(),
            last_frame: None,
            bodies: vec![Body::default()],
            wireframe: false,
            paused: false,
            bg_color: DVec3::default(),
            texture_key: "moon",
        }
    }
}

impl<'a> State<'a> {
    pub fn input(&mut self, event: &WindowEvent) {
        // We have no events to handle currently
        match event {
            WindowEvent::KeyboardInput { input, .. }
                if input.virtual_keycode == Some(VirtualKeyCode::Space)
                    && input.state == ElementState::Released =>
            {
                self.paused = !self.paused;
            }
            WindowEvent::KeyboardInput { input, .. }
                if input.virtual_keycode == Some(VirtualKeyCode::W)
                    && input.state == ElementState::Released =>
            {
                self.wireframe = !self.wireframe;
            }
            WindowEvent::KeyboardInput { input, .. }
            if input.virtual_keycode == Some(VirtualKeyCode::T)
                && input.state == ElementState::Released =>
            {
                self.texture_key = match self.texture_key {
                    "moon" => { "cookie" },
                    _ => { "moon" }
                };
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.mouse_pos = DVec2::new(position.x, position.y);
            }
            WindowEvent::Resized(size) => {
                self.window_size = UVec2::new(size.width, size.height);
            }
            _ => {}
        }
    }

    pub fn update(&mut self) {
        // Remain paused
        if self.paused {
            self.last_frame = Some(Instant::now());
            self.bg_color = DVec3::default();
            return;
        }

        match self.last_frame {
            Some(last_frame) => {
                let now = Instant::now();
                let dt = now - last_frame;

                // Update background color
                self.bg_color =
                    (self.mouse_pos / self.window_size.as_dvec2()).extend(0.0);

                // Simulation logic
                let dt_f32 = dt.as_secs_f32();
                for body in self.bodies.iter_mut() {
                    body.update(dt_f32);
                }
                self.last_frame = Some(now);
            }
            None => {
                self.last_frame = Some(Instant::now());
            }
        }
    }
}
