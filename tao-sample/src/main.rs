use std::path::Path;

use tao::{
    event::Event,
    event_loop::{ControlFlow, EventLoop},
    window::{Icon, WindowBuilder},
};

fn main() {
    env_logger::init();
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/src/icon.png");
    println!("{path}");
    let icon = load_icon(Path::new(path));

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Espanso")
        .with_window_icon(Some(icon))
        .build(&event_loop)
        .unwrap();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        if let Event::WindowEvent { event, .. } = event {
            use tao::event::WindowEvent::*;
            match event {
                CloseRequested => *control_flow = ControlFlow::Exit,
                DroppedFile(path) => {
                    window.set_window_icon(Some(load_icon(&path)));
                }
                _ => (),
            }
        }
    });
}

fn load_icon(path: &Path) -> Icon {
    let (icon_rgba, icon_width, icon_height) = {
        // alternatively, you can embed the icon in the binary through `include_bytes!` macro and use `image::load_from_memory`
        let image = image::open(path)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon")
}
