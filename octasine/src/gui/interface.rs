use iced::{Element, Sandbox, Text};
use iced_native::{UserInterface, Cache, Size};
use iced_wgpu::{wgpu, Renderer, Settings, Target};

use vst_window::{EditorWindow, EventSource};

use super::{GUI_WIDTH, GUI_HEIGHT};


pub struct Interface {
    // window: EditorWindow,
    event_source: EventSource,
    device: wgpu::Device,
    queue: wgpu::Queue,
    // encoder: wgpu::CommandEncoder,
    // surface: wgpu::Surface,
    // swap_chain: wgpu::SwapChain,
    renderer: Renderer,
    hello: Hello,
    cache: Option<Cache>,
    window_size: Size,
}


impl Interface {
    pub fn new(
        window: EditorWindow,
        event_source: EventSource,
    ) -> Self {
        let (mut device, queue) = Self::get_device_and_queue();
        // let surface = wgpu::Surface::create(&window);
        let renderer = Renderer::new(&mut device, Settings::default());

        /*
        let swap_chain_descriptor = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT, // ??
            format: wgpu::TextureFormat::Bgra8Unorm,
            width: GUI_WIDTH as u32,
            height: GUI_HEIGHT as u32,
            present_mode: wgpu::PresentMode::NoVsync, // ??
        };

        let swap_chain = device.create_swap_chain(&surface, &swap_chain_descriptor);
        */

        Self {
            // window,
            event_source,
            device,
            queue,
            // surface,
            renderer,
            // swap_chain,
            hello: Hello::new(),
            cache: Some(Cache::new()),
            window_size: Size::new(GUI_WIDTH as f32, GUI_HEIGHT as f32)
        }
    }

    fn get_device_and_queue() -> (wgpu::Device, wgpu::Queue) {
        let adapter = wgpu::Adapter::request(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::Default,
            backends: wgpu::BackendBit::PRIMARY,
        }).expect("Request adapter");

        adapter.request_device(&wgpu::DeviceDescriptor {
            extensions: wgpu::Extensions {
                anisotropic_filtering: false,
            },
            limits: wgpu::Limits::default(),
        })
    }

    pub fn process_events(&mut self){
        while let Some(event) = self.event_source.poll_event(){
            #[cfg(feature = "logging")]
            ::log::info!("event: {:?}", event);
        }

        let cache = self.cache.take().unwrap_or_else(|| Cache::default());

        let mut encoder = self.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor { todo: 0 },
        );

        let user_interface = UserInterface::build(
            self.hello.view(),
            self.window_size,
            cache,
            &mut self.renderer
        );

        let mouse_cursor = user_interface.draw(&mut self.renderer);

        self.cache = Some(user_interface.into_cache());

        /*

        let swap_chain_output = self.swap_chain.get_next_texture();

        let target = Target {
            texture: &swap_chain_output.view,
            viewport: &iced_wgpu::Viewport::new(GUI_WIDTH as u32, GUI_HEIGHT as u32),
        };

        let output = (iced_wgpu::Primitive::None , iced_native::MouseCursor::OutOfBounds);

        self.renderer.draw::<String>(
            &mut self.device,
            &mut encoder,
            target,
            &output,
            1.0, // ?
            &[],
        );

        */

        self.queue.submit(&[encoder.finish()]);

        // todo: set cursor icon??
    }
}


pub struct Hello;


impl Sandbox for Hello {
    type Message = ();

    fn new() -> Self {
        Self
    }

    fn title(&self) -> String {
        "Hello".to_string()
    }

    fn update(&mut self, message: Self::Message) {
        
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        Text::new("Hello").into()
    }
}