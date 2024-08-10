use glium::{winit};

fn main() {
    // handles events like mouse clicks
    let event_loop = glium::winit::event_loop::EventLoopBuilder::new().build().expect("event loop building");
    // window and display for drawing frames 
    let (_window, display) = glium::backend::glutin::SimpleWindowBuilder::new().build(&event_loop);

    // start loop
    let _ = event_loop.run(move |event, window_target| {
        match event {
            glium::winit::event::Event::WindowEvent { event, .. } => match event {
                glium::winit::event::WindowEvent::CloseRequested => window_target.exit(),
                _ => (),
            },
            _ => (),
        };
    });
}
