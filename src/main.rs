extern crate clap;
extern crate glutin;
extern crate servo;

use clap::{App, Arg};
use servo::gl;
use glutin::GlContext;
use servo::BrowserId;
use servo::compositing::compositor_thread::EventLoopWaker;
use servo::compositing::windowing::{WindowEvent, WindowMethods};
use servo::euclid::{Point2D, ScaleFactor, Size2D, TypedPoint2D, TypedRect, TypedSize2D,
                    TypedVector2D};
use servo::ipc_channel::ipc;
use servo::msg::constellation_msg::{Key, KeyModifiers, TopLevelBrowsingContextId};
use servo::net_traits::net_error_list::NetError;
use servo::script_traits::{LoadData, TouchEventType};
use servo::servo_config::opts;
use servo::servo_config::resource_files::set_resources_path;
use servo::servo_geometry::DeviceIndependentPixel;
use servo::servo_url::ServoUrl;
use servo::style_traits::DevicePixel;
use servo::style_traits::cursor::Cursor;
use std::env;
use std::rc::Rc;
use std::sync::Arc;

// servo-embedding-example was introduced here:
// https://groups.google.com/forum/#!msg/mozilla.dev.servo/20lkEsRI-ZI/RbEaVG_MAAAJ

pub struct GlutinEventLoopWaker {
    proxy: Arc<glutin::EventsLoopProxy>,
}

impl EventLoopWaker for GlutinEventLoopWaker {
    // Use by servo to share the "event loop waker" across threads
    fn clone(&self) -> Box<EventLoopWaker + Send> {
        Box::new(GlutinEventLoopWaker { proxy: self.proxy.clone() })
    }
    // Called by servo when the main thread needs to wake up
    fn wake(&self) {
        self.proxy.wakeup().expect("wakeup eventloop failed");
    }
}

trait AbstractWindow: WindowMethods {
    fn resize(&self, width: u32, height: u32);
}
struct Window {
    glutin_window: glutin::GlWindow,
    waker: Box<EventLoopWaker>,
    gl: Rc<gl::Gl>,
}
impl AbstractWindow for Window {
    fn resize(&self, width: u32, height: u32) {
        self.glutin_window.resize(width, height);
    }
}
struct HeadlessWindow {
    headless_context: glutin::HeadlessContext,
    gl: Rc<gl::Gl>,
}
impl AbstractWindow for HeadlessWindow {
    fn resize(&self, width: u32, height: u32) {
        self.headless_context.resize(width, height);
    }
}

trait ServoTrait {
    fn handle_events(&mut self, events: Vec<WindowEvent>) -> bool;
    fn resize(&self, width: u32, height: u32);
}
struct ServoTraitObject<T: AbstractWindow + 'static> {
    window: Rc<T>,
    servo: Option<servo::Servo<T>>,
}
impl<T: AbstractWindow + 'static> ServoTrait for ServoTraitObject<T> {
    fn handle_events(&mut self, events: Vec<WindowEvent>) -> bool {
        self.servo.as_mut().expect("handle_events after deinit").handle_events(events)
    }
    fn resize(&self, width: u32, height: u32) { self.window.resize(width, height); }
}
impl<T: AbstractWindow + 'static> Drop for ServoTraitObject<T> {
    fn drop(&mut self) {
        let mut servo_opt: Option<servo::Servo<T>> = None;
        std::mem::swap(&mut servo_opt, &mut self.servo);
        if let Some(servo) = servo_opt {
            servo.deinit();
        }
    }
}


fn main() {
    let matches = App::new("My Super Program")
                          .version("1.0")
                          .author("Yonathan. <yonathan@gmail.com>")
                          .about("scrape the web")
                          .arg(Arg::with_name("headless")
                               .long("headless2")
                               .help("Disable head"))
                          .get_matches();
    let headless: bool = matches.is_present("headless");

    println!("Servo version: {}", servo::config::servo_version());


    // let gl_version = glutin::GlRequest::Latest;
    let gl_version = glutin::GlRequest::Specific(glutin::Api::OpenGl, (3, 2));

    let path = env::current_dir().unwrap().join("resources");
    let path = path.to_str().unwrap().to_string();
    set_resources_path(Some(path));
    if true {
        opts::set_defaults(opts::Opts { 
            headless: true,
            ..opts::default_opts()
        });
        println!("headless:true");
    } else {
        let args: Vec<String> = std::env::args().collect();
        opts::from_cmdline_args(&*args);
    }

    let (mut servo, event_loop_opt): (Box<ServoTrait>, Option<glutin::EventsLoop>) = if headless {
        let headless_test_context: glutin::HeadlessContext = glutin::HeadlessRendererBuilder::new(800, 600)
            .with_gl(glutin::GlRequest::Latest)
            .build()
            .expect("failed to create Latest headless context");
        let test_gl = unsafe {
            headless_test_context.make_current().expect("make_current failed");
            gl::GlFns::load_with(|s| {
                // println!("GlFns {} -> {:?}", s, headless_test_context.get_proc_address(s));
                headless_test_context.get_proc_address(s) as *const _
            })
        };
        println!("Latest has GL_MAJOR_VERSION {}, GL_MINOR_VERSION {}, GL_VERSION {}", test_gl.get_integer_v(gl::MAJOR_VERSION), test_gl.get_integer_v(gl::MINOR_VERSION), test_gl.get_string(gl::VERSION));
        println!("Shading language version: {}", test_gl.get_string(gl::SHADING_LANGUAGE_VERSION));

        let headless_context: glutin::HeadlessContext = glutin::HeadlessRendererBuilder::new(800, 600)
            .with_gl(gl_version)
            .build()
            .expect("failed to create headless context");
        let gl = unsafe {
            headless_context.make_current().expect("make_current failed");
            gl::GlFns::load_with(|s| headless_context.get_proc_address(s) as *const _)
        };
        println!("{:?} has GL_MAJOR_VERSION {}, GL_MINOR_VERSION {}, GL_VERSION {}", gl_version, gl.get_integer_v(gl::MAJOR_VERSION), gl.get_integer_v(gl::MINOR_VERSION), gl.get_string(gl::VERSION));
        println!("Shading language version: {}", gl.get_string(gl::SHADING_LANGUAGE_VERSION));
        let print_errors = |s: &str| {
            loop {
                let err = test_gl.get_error();
                if err != 0 {
                    println!("{} error {:04x}", s, err);
                } else {
                    break;
                }
            }
        };
        print_errors("after make_current");
        println!("Gl.get_type={:?}", gl.get_type());
        println!("MAX_TEXTURE_SIZE={}", gl.get_integer_v(gl::MAX_TEXTURE_SIZE));
        print_errors("after MAX_TEXTURE_SIZE");
        let window = Rc::new(HeadlessWindow {headless_context: headless_context, gl: gl});
        let servo = servo::Servo::<HeadlessWindow>::new(window.clone());
        print_errors("after create servo");
        let servo_box: Box<ServoTrait> = Box::new(ServoTraitObject { servo: Some(servo), window: window });
        print_errors("after servo_box");
        (servo_box, None)
    } else {
        let mut event_loop = glutin::EventsLoop::new();
        let builder = glutin::WindowBuilder::new().with_dimensions(800, 600);
        let context = glutin::ContextBuilder::new()
            .with_gl(gl_version)
            .with_vsync(true);
        let window = glutin::GlWindow::new(builder, context, &event_loop).unwrap();

        window.show();

        let gl = unsafe {
            window
                .context()
                .make_current()
                .expect("Couldn't make window current");
            gl::GlFns::load_with(|s| window.context().get_proc_address(s) as *const _)
        };

        let event_loop_waker =
            Box::new(GlutinEventLoopWaker { proxy: Arc::new(event_loop.create_proxy()) });
        let window: Rc<Window> = Rc::new(Window {
            glutin_window: window,
            waker: event_loop_waker,
            gl: gl,
        });
        let servo = servo::Servo::<Window>::new(window.clone());
        let servo_box: Box<ServoTrait> = Box::new(ServoTraitObject { servo: Some(servo), window: window });
        (servo_box, Some(event_loop))
    };

    let url = ServoUrl::parse("https://google.com").unwrap();
    let (sender, receiver) = ipc::channel().unwrap();
    servo.handle_events(vec![WindowEvent::NewBrowser(url, sender)]);
    let browser_id = receiver.recv().unwrap();
    servo.handle_events(vec![WindowEvent::SelectBrowser(browser_id)]);

    let mut pointer = (0.0, 0.0);


    if let Some(mut event_loop) = event_loop_opt {
        event_loop.run_forever(|event| {
            // Blocked until user event or until servo unblocks it
            match event {
                // This is the event triggered by GlutinEventLoopWaker
                glutin::Event::Awakened => {
                    servo.handle_events(vec![]);
                }

                // Mousemove
                glutin::Event::WindowEvent {
                    event: glutin::WindowEvent::CursorMoved { position: (x, y), .. }, ..
                } => {
                    pointer = (x, y);
                    let event = WindowEvent::MouseWindowMoveEventClass(TypedPoint2D::new(x as f32,
                                                                                        y as f32));
                    servo.handle_events(vec![event]);
                }

                // reload when R is pressed
                glutin::Event::WindowEvent {
                    event: glutin::WindowEvent::KeyboardInput {
                        input: glutin::KeyboardInput {
                            state: glutin::ElementState::Pressed,
                            virtual_keycode: Some(glutin::VirtualKeyCode::R),
                            ..
                        },
                        ..
                    },
                    ..
                } => {
                    let event = WindowEvent::Reload(browser_id);
                    servo.handle_events(vec![event]);
                }

                // Scrolling
                glutin::Event::WindowEvent {
                    event: glutin::WindowEvent::MouseWheel { delta, phase, .. }, ..
                } => {
                    let pointer = TypedPoint2D::new(pointer.0 as i32, pointer.1 as i32);
                    let (dx, dy) = match delta {
                        glutin::MouseScrollDelta::LineDelta(dx, dy) => {
                            (dx, dy * 38.0 /*line height*/)
                        }
                        glutin::MouseScrollDelta::PixelDelta(dx, dy) => (dx, dy),
                    };
                    let scroll_location =
                        servo::webrender_api::ScrollLocation::Delta(TypedVector2D::new(dx, dy));
                    let phase = match phase {
                        glutin::TouchPhase::Started => TouchEventType::Down,
                        glutin::TouchPhase::Moved => TouchEventType::Move,
                        glutin::TouchPhase::Ended => TouchEventType::Up,
                        glutin::TouchPhase::Cancelled => TouchEventType::Up,
                    };
                    let event = WindowEvent::Scroll(scroll_location, pointer, phase);
                    servo.handle_events(vec![event]);
                }
                glutin::Event::WindowEvent {
                    event: glutin::WindowEvent::Resized(width, height), ..
                } => {
                    let event = WindowEvent::Resize;
                    servo.handle_events(vec![event]);
                    servo.resize(width, height);
                }
                _ => {}
            }
            glutin::ControlFlow::Continue
        });
    }
}

impl WindowMethods for Window {
    fn prepare_for_composite(&self, _width: usize, _height: usize) -> bool {
        true
    }

    fn present(&self) {
        self.glutin_window.swap_buffers().unwrap();
    }

    fn supports_clipboard(&self) -> bool {
        false
    }

    fn create_event_loop_waker(&self) -> Box<EventLoopWaker> {
        self.waker.clone()
    }

    fn gl(&self) -> Rc<gl::Gl> {
        self.gl.clone()
    }

    fn hidpi_factor(&self) -> ScaleFactor<f32, DeviceIndependentPixel, DevicePixel> {
        ScaleFactor::new(self.glutin_window.hidpi_factor())
    }

    fn framebuffer_size(&self) -> TypedSize2D<u32, DevicePixel> {
        let (width, height) = self.glutin_window.get_inner_size().unwrap();
        let scale_factor = self.glutin_window.hidpi_factor() as u32;
        TypedSize2D::new(scale_factor * width, scale_factor * height)
    }

    fn window_rect(&self) -> TypedRect<u32, DevicePixel> {
        TypedRect::new(TypedPoint2D::new(0, 0), self.framebuffer_size())
    }

    // TODO(yonran): are these screen pixels?
    fn screen_size(&self, _ctx: TopLevelBrowsingContextId) -> Size2D<u32> {
        TypedSize2D::new(3, 3)
    }
    fn screen_avail_size(&self, _ctx: TopLevelBrowsingContextId) -> Size2D<u32> {
        TypedSize2D::new(3, 3)
    }

    fn size(&self) -> TypedSize2D<f32, DeviceIndependentPixel> {
        let (width, height) = self.glutin_window.get_inner_size().unwrap();
        TypedSize2D::new(width as f32, height as f32)
    }

    fn client_window(&self, _id: BrowserId) -> (Size2D<u32>, Point2D<i32>) {
        let (width, height) = self.glutin_window.get_inner_size().unwrap();
        let (x, y) = self.glutin_window.get_position().unwrap();
        (Size2D::new(width, height), Point2D::new(x as i32, y as i32))
    }

    fn set_inner_size(&self, _id: BrowserId, _size: Size2D<u32>) {}

    fn set_position(&self, _id: BrowserId, _point: Point2D<i32>) {}

    fn set_fullscreen_state(&self, _id: BrowserId, _state: bool) {}

    fn set_page_title(&self, _id: BrowserId, title: Option<String>) {
        self.glutin_window
            .set_title(match title {
                           Some(ref title) => title,
                           None => "",
                       });
    }

    fn status(&self, _id: BrowserId, _status: Option<String>) {}

    fn allow_navigation(&self, _id: BrowserId, _url: ServoUrl, chan: ipc::IpcSender<bool>) {
        chan.send(true).ok();
    }

    fn load_start(&self, _id: BrowserId) {}

    fn load_end(&self, _id: BrowserId) {}

    fn load_error(&self, _id: BrowserId, _: NetError, _url: String) {}

    fn head_parsed(&self, _id: BrowserId) {}

    fn history_changed(&self, _id: BrowserId, _entries: Vec<LoadData>, _current: usize) {}

    fn set_cursor(&self, cursor: Cursor) {
        let cursor = match cursor {
            Cursor::Pointer => glutin::MouseCursor::Hand,
            _ => glutin::MouseCursor::Default,
        };
        self.glutin_window.set_cursor(cursor);
    }

    fn set_favicon(&self, _id: BrowserId, _url: ServoUrl) {}

    fn handle_key(&self,
                  _id: Option<BrowserId>,
                  _ch: Option<char>,
                  _key: Key,
                  _mods: KeyModifiers) {
    }
}

#[derive(Clone, Copy)]
struct NoEventLoopWaker;
impl EventLoopWaker for NoEventLoopWaker {
    fn clone(&self) -> Box<EventLoopWaker + Send> {Box::new(NoEventLoopWaker)}
    fn wake(&self) {}
}
impl WindowMethods for HeadlessWindow {
    fn prepare_for_composite(&self, _width: usize, _height: usize) -> bool {
        true
    }

    fn present(&self) {
        self.headless_context.swap_buffers().unwrap();
    }

    fn supports_clipboard(&self) -> bool {
        false
    }

    fn create_event_loop_waker(&self) -> Box<EventLoopWaker> {
        Box::new(NoEventLoopWaker)
    }

    fn gl(&self) -> Rc<gl::Gl> {
        self.gl.clone()
    }

    fn hidpi_factor(&self) -> ScaleFactor<f32, DeviceIndependentPixel, DevicePixel> {
        ScaleFactor::new(2.0)
    }

    fn framebuffer_size(&self) -> TypedSize2D<u32, DevicePixel> {
        // let (width, height) = self.glutin_window.get_inner_size().unwrap();
        // let scale_factor = self.glutin_window.hidpi_factor() as u32;
        let (width, height) = (800u32, 600u32); // TODO
        let scale_factor: u32 = self.hidpi_factor().get() as u32;
        TypedSize2D::new(scale_factor * width, scale_factor * height)
    }

    fn window_rect(&self) -> TypedRect<u32, DevicePixel> {
        TypedRect::new(TypedPoint2D::new(0, 0), self.framebuffer_size())
    }

    // TODO(yonran): are these screen pixels?
    fn screen_size(&self, _ctx: TopLevelBrowsingContextId) -> Size2D<u32> {
        TypedSize2D::new(3, 3)
    }
    fn screen_avail_size(&self, _ctx: TopLevelBrowsingContextId) -> Size2D<u32> {
        TypedSize2D::new(3, 3)
    }

    fn size(&self) -> TypedSize2D<f32, DeviceIndependentPixel> {
        // let (width, height) = self.glutin_window.get_inner_size().unwrap();
        let (width, height) = (800, 600);
        TypedSize2D::new(width as f32, height as f32)
    }

    fn client_window(&self, _id: BrowserId) -> (Size2D<u32>, Point2D<i32>) {
        // let (width, height) = self.glutin_window.get_inner_size().unwrap();
        // let (x, y) = self.glutin_window.get_position().unwrap();
        let (width, height) = (800, 600);
        let (x, y) = (0, 0);
        (Size2D::new(width, height), Point2D::new(x as i32, y as i32))
    }

    fn set_inner_size(&self, _id: BrowserId, _size: Size2D<u32>) {}

    fn set_position(&self, _id: BrowserId, _point: Point2D<i32>) {}

    fn set_fullscreen_state(&self, _id: BrowserId, _state: bool) {}

    fn set_page_title(&self, _id: BrowserId, _title: Option<String>) {
        // self.glutin_window
        //     .set_title(match title {
        //                    Some(ref title) => title,
        //                    None => "",
        //                });
    }

    fn status(&self, _id: BrowserId, _status: Option<String>) {}

    fn allow_navigation(&self, _id: BrowserId, _url: ServoUrl, chan: ipc::IpcSender<bool>) {
        chan.send(true).ok();
    }

    fn load_start(&self, _id: BrowserId) {}

    fn load_end(&self, _id: BrowserId) {}

    fn load_error(&self, _id: BrowserId, _: NetError, _url: String) {}

    fn head_parsed(&self, _id: BrowserId) {}

    fn history_changed(&self, _id: BrowserId, _entries: Vec<LoadData>, _current: usize) {}

    fn set_cursor(&self, _cursor: Cursor) {
        // let cursor = match cursor {
        //     Cursor::Pointer => glutin::MouseCursor::Hand,
        //     _ => glutin::MouseCursor::Default,
        // };
        // self.glutin_window.set_cursor(cursor);
    }

    fn set_favicon(&self, _id: BrowserId, _url: ServoUrl) {}

    fn handle_key(&self,
                  _id: Option<BrowserId>,
                  _ch: Option<char>,
                  _key: Key,
                  _mods: KeyModifiers) {
    }
}
