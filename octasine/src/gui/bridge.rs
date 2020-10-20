/// Bridge vst_window and iced
///
/// Heavily based on code by https://github.com/BillyDM

use iced_graphics::Viewport;
use iced_native::{program, Color, Command, Debug, Element, Point, Size, Event};
use vst_window::{EditorWindow, EventSource};


pub type Renderer = iced_wgpu::Renderer;
type Compositor = iced_wgpu::window::Compositor;


pub trait Application: Sized {
    type Message: std::fmt::Debug + Send;

    fn new() -> Self;

    fn title(&self) -> String;

    fn update(&mut self, message: Self::Message) -> Command<Self::Message>;

    fn view(&mut self) -> Element<'_, Self::Message, Renderer>;

    fn subscription(&self) -> iced_native::Subscription<Self::Message> {
        iced_native::Subscription::none()
    }

    fn background_color() -> Color {
        Color::WHITE
    }

    fn compositor_settings() -> iced_wgpu::Settings {
        iced_wgpu::Settings::default()
    }
}


struct IcedProgram<A: Application> {
    pub user_app: A,
}


impl<A: Application> iced_native::Program for IcedProgram<A> {
    type Renderer = Renderer;
    type Message = A::Message;

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        self.user_app.update(message)
    }

    fn view(&mut self) -> Element<'_, Self::Message, Self::Renderer> {
        self.user_app.view()
    }
}


pub struct Handler<A: Application + 'static> {
    event_source: vst_window::EventSource,
    iced_state: iced_native::program::State<IcedProgram<A>>,
    cursor_position: iced_native::Point,
    debug: iced_native::Debug,
    viewport: iced_graphics::Viewport,
    compositor: Compositor,
    renderer: Renderer,
    // surface: <Compositor as iced_graphics::window::Compositor>::Surface,
    swap_chain: <Compositor as iced_graphics::window::Compositor>::SwapChain,
    background_color: Color,
    redraw_requested: bool,
    // window_size: iced_native::Size<u32>,
    // scale_factor: f64,
}


impl <A: Application + 'static>Handler<A> {
    pub fn build(
        mut window: EditorWindow,
        event_source: EventSource,
        width: u32,
        height: u32,
    ) -> Self {
        use iced_graphics::window::Compositor as IGCompositor;

        // let window_info = window.window_info();
        let scale_factor = 1.0; // FIXME

        // iced's debugger
        let mut debug = Debug::new();

        let window_size = Size::new(width, height);

        // The iced_graphics viewport
        let viewport =
            Viewport::with_physical_size(window_size, scale_factor);

        // Get the compositor settings that your user has requested (such as antialiasing, etc.)
        let compositor_settings = A::compositor_settings();

        // Create the iced compositor and renderer.
        let (mut compositor, mut renderer) =
            <Compositor as IGCompositor>::new(compositor_settings).unwrap();

        // Create the wgpu surface :D
        // The baseview `window` extends `raw_window_handle::HasRawWindowHandle`.
        let surface = compositor.create_surface(&mut window);

        // Create the wgpu swapchain
        let swap_chain = compositor.create_swap_chain(
            &surface,
            window_size.width,
            window_size.height,
        );

        // Initialize user program
        let user_app = A::new();
        let iced_program = IcedProgram { user_app };

        let background_color = A::background_color();

        // Initialize iced's built-in state
        let iced_state = program::State::new(
            iced_program,
            viewport.logical_size(),
            Point::new(-1.0, -1.0),
            &mut renderer,
            &mut debug,
        );

        Handler {
            event_source,
            iced_state,
            cursor_position: Point::new(-1.0, -1.0),
            debug,
            viewport,
            compositor,
            renderer,
            swap_chain,
            redraw_requested: true,
            background_color,
        }
    }

    pub fn process_events(&mut self){
        use vst_window::WindowEvent::*;

        while let Some(event) = self.event_source.poll_event(){
            let event = match event {
                CursorMovement(x, y) => {
                    self.cursor_position.x = x;
                    self.cursor_position.y = y;

                    Event::Mouse(iced::mouse::Event::CursorMoved {
                        x,
                        y
                    })
                },
                MouseClick(button) => {
                    let button = match button {
                        vst_window::MouseButton::Left => iced::mouse::Button::Left,
                        vst_window::MouseButton::Right => iced::mouse::Button::Right,
                        vst_window::MouseButton::Middle => iced::mouse::Button::Middle,
                    };

                    Event::Mouse(iced::mouse::Event::ButtonPressed(button))
                },
                MouseRelease(button) => {
                    let button = match button {
                        vst_window::MouseButton::Left => iced::mouse::Button::Left,
                        vst_window::MouseButton::Right => iced::mouse::Button::Right,
                        vst_window::MouseButton::Middle => iced::mouse::Button::Middle,
                    };

                    Event::Mouse(iced::mouse::Event::ButtonReleased(button))
                }
            };

            self.iced_state.queue_event(event);
            self.redraw_requested = true;
        }

        self.on_frame();
    }

    fn on_frame(&mut self) {
        use iced_graphics::window::Compositor as IGCompositor;

        if self.redraw_requested {
            // Update iced state, draining all of the events stored in its queue.
            // You can do this whenever, but I decided to do this in baseview's on_frame() event.
            // Keep in mind that each call to `iced_state.update()` will recreate the widget tree,
            // so you want to do this as little as possible.
            let _ = self.iced_state.update(
                self.viewport.logical_size(),
                self.cursor_position,
                None, // clipboard
                &mut self.renderer,
                &mut self.debug,
            );

            // Iced's debug log
            self.debug.render_started();

            // Send all of the information to the compositor and draw everything to the screen. :D
            // This also returns the mouse cursor to display (Baseview doesn't support this yet).
            let _new_mouse_interaction = self.compositor.draw(
                &mut self.renderer,
                &mut self.swap_chain,
                &self.viewport,
                self.background_color,
                self.iced_state.primitive(),
                &self.debug.overlay(),
            );

            self.debug.render_finished();

            self.redraw_requested = false;
        }
    }
}