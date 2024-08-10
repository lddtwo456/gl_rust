#[macro_use]
extern crate glium;
use glium::{winit, winit::event, backend, Surface};

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}
implement_vertex!(Vertex, position);

#[allow(deprecated)]
fn main() {
    // handles events like mouse clicks
    let event_loop = winit::event_loop::EventLoopBuilder::new().build().expect("event loop building");
    // window and display for drawing frames 
    let (window, display) = backend::glutin::SimpleWindowBuilder::new().build(&event_loop);

    // triangle
    let v1 = Vertex { position: [-0.5, -0.5] };
    let v2 = Vertex { position: [ 0.0,  0.5] };
    let v3 = Vertex { position: [ 0.5, -0.25] };
    let shape = vec![v1, v2, v3];

    // make buffers
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let index_buffer = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    // shaders
    let vertex_shader_src = r#"
    #version 140

    in vec2 position;

    uniform float x;

    void main() {
        vec2 pos = position;
        pos.x += x*pos.y;
        gl_Position = vec4(pos, 0.0, 1.0);
    }
    "#;

    let fragment_shader_src = r#"
    #version 140

    out vec4 color;

    void main() {
        color = vec4(1.0, 0.0, 0.0, 1.0);
    }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let mut t: f32 = 0.0;

    // start loop
    let _ = event_loop.run(move |event, window_target| {
        match event {
            event::Event::WindowEvent { event, .. } => match event {
                event::WindowEvent::CloseRequested => window_target.exit(),
                event::WindowEvent::Resized(window_size) => {
                    display.resize(window_size.into());
                },
                event::WindowEvent::RedrawRequested => {
                    // calls when we want to draw each frame
                    t += 0.001;
                    let x_off = t.sin() * 0.5;

                    // set up frame
                    let mut frame = display.draw();
                    frame.clear_color(0.0, 1.0, 1.0, 1.0);

                    // draw
                    frame.draw(&vertex_buffer, &index_buffer, &program, &uniform! { x: x_off },
                               &Default::default()).unwrap();
                    frame.finish().unwrap();
                },
                _ => (),
            },
            event::Event::AboutToWait => {
                window.request_redraw();
            },
            _ => (),
        };
    });
}
