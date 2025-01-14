mod dom;
mod render;
mod runtime;
mod sim;

use gloo_console::log;
use render::WgpuContext;
use wasm_bindgen::prelude::*;
use winit::dpi::LogicalSize;
use winit::platform::web::WindowBuilderExtWebSys;
use winit::{event_loop::EventLoop, window::WindowBuilder};

use crate::dom::Dom;
use crate::runtime::Runtime;

#[wasm_bindgen(start)]
pub async fn run() {
    // Redirect panics to the console (debugging)
    console_error_panic_hook::set_once();

    let dom = Dom::new();
    let canvas = dom::get_canvas();
    let (width, height) = (canvas.client_width(), canvas.client_height());
    log!("Acquired DOM elements");

    // Connect graphics card to window
    let mut context = WgpuContext::new(&canvas).await;
    log!("Acquired graphics context");

    // Create window
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_canvas(Some(canvas))
        .build(&event_loop)
        .and_then(|w| {
            // Set initial view port -- ** This isn't what we want! **
            // We want the canvas to always fit to the document.
            w.set_inner_size(LogicalSize::new(width, height));
            Ok(w)
        })
        .expect("Could not build window");
    log!("Created window");

    // Load shaders
    context.add_shader("vert", include_str!("../assets/shaders/vert.wgsl"));
    context.add_shader("frag", include_str!("../assets/shaders/frag.wgsl"));
    context.add_shader(
        "wireframe.vert",
        include_str!("../assets/shaders/wireframe.vert.wgsl"),
    );
    context.add_shader(
        "wireframe.frag",
        include_str!("../assets/shaders/wireframe.frag.wgsl"),
    );
    context.add_shader(
        "world.vert",
        include_str!("../assets/shaders/world.vert.wgsl"),
    );
    context.add_shader(
        "world.frag",
        include_str!("../assets/shaders/world.frag.wgsl"),
    );
    log!("Loaded shaders");

    // Load textures
    context
        .add_texture("disco", include_bytes!("../assets/textures/disco.jpg"));
    context.add_texture("rust", include_bytes!("../assets/textures/rust.png"));
    log!("Loaded textures");

    // Run program
    let mut runtime = Runtime::new(context, window, dom);
    log!("Starting...");
    event_loop.run(move |event, target, control_flow| {
        runtime.main_loop(event, target, control_flow)
    });
}
