use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

fn main() {
    // WGPU logs via this crate. We must call init to enable it
    env_logger::init();

    // Requires eventloop::EventLoop
    let event_loop = EventLoop::new().unwrap();
    // POLL Runs event loop continuously, even when no events were registered. Good for games, bad for UI
    event_loop.set_control_flow(ControlFlow::Poll);

    // Input the event loop and this is where the window will send its events
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let size = window.inner_size();

    // Handle to our GPU
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all(),
        ..Default::default()
    });

    // Tutorial puts this in an unsafe block, but it seems safe to me?
    let surface = instance.create_surface(&window).unwrap();

    // Create the adaptor!
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        })
        .await
        .unwrap(); // RAAAAAAGH why is this async? Who would ever want to be doing something else while the adaptor is requested?

    event_loop
        // Run supply's the event and event_loop_control_target inputs
        .run(move |event, event_loop_window_target| {
            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    .. // We don't give a shit what the window ID is, so we dont put it in the pattern
                } => {
                    // The close button was pressed, close the window
                    event_loop_window_target.exit();
                }
                Event::AboutToWait => {
                    // Update
                }
                Event::WindowEvent {
                    event: WindowEvent::RedrawRequested,
                    ..
                } => {
                    // Draw
                }
                _ => (),
            }
        })
        .unwrap();
}

struct State<'window> {
    // The surface is the texture we draw to
    surface: wgpu::Surface<'window>,
    // The device is how we access the GPU
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    // The window must be declared after the surface so
    // it gets dropped after it as the surface contains
    // unsafe references to the window's resources.
    window: Window,
}

impl<'window> State<'window> {
    // Creating some of the wgpu types requires async code
    async fn new(window: Window) -> Self {
        todo!()
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        todo!()
    }

    fn input(&mut self, event: &WindowEvent) -> bool {
        todo!()
    }

    fn update(&mut self) {
        todo!()
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        todo!()
    }
}
