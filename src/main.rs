use glium::{winit, backend};

#[allow(deprecated)]
fn main() {
    // handles events like mouse clicks
    let event_loop = winit::event_loop::EventLoopBuilder::new().build().expect("event loop building");
    // window and display for drawing frames 
    let (_window, display) = backend::glutin::SimpleWindowBuilder::new().build(&event_loop);

    // start loop
    let _ = event_loop.run(move |event, window_target| {
        match event {
            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::CloseRequested => window_target.exit(),
                _ => (),
            },
            _ => (),
        };
    });
}
