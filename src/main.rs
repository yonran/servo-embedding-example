extern crate clap;
extern crate env_logger;
extern crate glutin;
extern crate image;
#[macro_use] extern crate log;
extern crate servo;

mod loggl;

use clap::App;
use servo::gl;
use glutin::GlContext;
use servo::BrowserId;
use servo::compositing::compositor_thread::EventLoopWaker;
use servo::compositing::windowing::{WindowEvent, WindowMethods};
use servo::euclid::{Point2D, ScaleFactor, Size2D, TypedPoint2D, TypedRect, TypedSize2D};
use servo::gl::Gl;
use servo::ipc_channel::ipc;
use servo::msg::constellation_msg::{Key, KeyModifiers, TopLevelBrowsingContextId};
use servo::net_traits::net_error_list::NetError;
use servo::script_traits::{LoadData};
use servo::servo_config::opts;
use servo::servo_config::resource_files::set_resources_path;
use servo::servo_geometry::DeviceIndependentPixel;
use servo::servo_url::ServoUrl;
use servo::style_traits::DevicePixel;
use servo::style_traits::cursor::Cursor;
use std::env;
use std::fs::File;
use std::io::Write;
use std::rc::Rc;
use std::sync::Arc;
use image::png::PNGEncoder;

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
    fn screenshot(&self) -> Vec<u8> {
        let device_size = self.framebuffer_size();
        let device_size_i32 = device_size.to_i32();
        // ReadPixels returns pixels from bottom left
        // based on webrender/wrench/png.rs
        let pixels_upsidedown = self.gl().read_pixels(0, 0, device_size_i32.width, device_size_i32.height, gl::RGBA, gl::UNSIGNED_BYTE);
        let mut buffer = image::RgbaImage::from_raw(
            device_size.width,
            device_size.height,
            pixels_upsidedown,
        ).expect("bug: unable to construct image buffer");
        // flip image vertically
        buffer = image::imageops::flip_vertical(&buffer);
        buffer.into_vec()
    }
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
    device_width: u32,
    device_height: u32,
    highdpi_factor: u32,
    logical_width: u32,
    logical_height: u32,
}
impl AbstractWindow for HeadlessWindow {
    fn resize(&self, width: u32, height: u32) {
        // Note: HeadlessContext cannot resize so this is a no-op
        warn!("Resize requested to width: {}, height: {}, but headless does not support resize", width, height);
        self.headless_context.resize(width, height);
    }
}

trait ServoAndWindowTrait {
    fn handle_events(&mut self, events: Vec<WindowEvent>) -> bool;
    fn resize(&self, width: u32, height: u32);
    fn window(&self) -> &AbstractWindow;
    fn repaint(&mut self);
}
struct ServoAndWindow<T: AbstractWindow + 'static> {
    window: Rc<T>,
    servo: Option<servo::Servo<T>>,
}
impl<T: AbstractWindow + 'static> ServoAndWindowTrait for ServoAndWindow<T> {
    fn handle_events(&mut self, events: Vec<WindowEvent>) -> bool {
        self.servo.as_mut().expect("handle_events after deinit").handle_events(events)
    }
    fn resize(&self, width: u32, height: u32) { self.window.resize(width, height); }
    fn window(&self) -> &AbstractWindow {&*self.window}
    fn repaint(&mut self) {
        if let Some(servo) = self.servo.as_mut() {
            servo.repaint_synchronously();
        }
    }
}
impl<T: AbstractWindow + 'static> Drop for ServoAndWindow<T> {
    fn drop(&mut self) {
        let mut servo_opt: Option<servo::Servo<T>> = None;
        std::mem::swap(&mut servo_opt, &mut self.servo);
        if let Some(servo) = servo_opt {
            info!("servo deinit start");
            servo.deinit();
            info!("servo deinit complete");
        }
    }
}

fn main() {
    env_logger::init().unwrap();
    let _matches = App::new("My Super Program")
                          .version("1.0")
                          .author("Yonathan. <yonathan@gmail.com>")
                          .about("scrape the web")
                          .get_matches();

    println!("Servo version: {}", servo::config::servo_version());

    let gl_version = glutin::GlRequest::Latest;
    // let gl_version = glutin::GlRequest::Specific(glutin::Api::OpenGl, (3, 2));

    let path = env::current_dir().unwrap().join("resources");
    let path = path.to_str().unwrap().to_string();
    set_resources_path(Some(path));
    if true {
        opts::set_defaults(opts::Opts { 
            headless: true,
            ..opts::default_opts()
        });
    } else {
        let args: Vec<String> = std::env::args().collect();
        opts::from_cmdline_args(&*args);
    }

    // aka backingScaleFactor in OSX AppKit NSWindow, devicePixelRatio in CSSOM
    let highdpi_factor: u32 = 2;
    let (logical_width, logical_height): (u32, u32) = (1024, 768);
    let (device_width, device_height): (u32, u32) = (logical_width * highdpi_factor, logical_height * highdpi_factor);

    let mut servo: Box<ServoAndWindowTrait> = {
        let headless_context: glutin::HeadlessContext = glutin::HeadlessRendererBuilder::new(device_width, device_height)
            .with_gl(gl_version)
            .build()
            .expect("failed to create headless context");
        let gl = loggl::LogGl::new(unsafe {
            headless_context.make_current().expect("make_current failed");
            gl::GlFns::load_with(|s| headless_context.get_proc_address(s) as *const _)
        });
        println!("{:?} has Vendor: {}, Renderer {}, Version {} (MajorVersion {}, MinorVersion {})", gl_version, gl.get_string(gl::VENDOR), gl.get_string(gl::RENDERER), gl.get_string(gl::VERSION), gl.get_integer_v(gl::MAJOR_VERSION), gl.get_integer_v(gl::MINOR_VERSION));
        println!("Shading language version: {}", gl.get_string(gl::SHADING_LANGUAGE_VERSION));
        {
            let print_errors = |s: &str| {
                loop {
                    let err = gl.get_error();
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
        }
        let window = Rc::new(HeadlessWindow {
            headless_context, gl,
            logical_width, logical_height,
            device_width, device_height,
            highdpi_factor});
        let servo = servo::Servo::<HeadlessWindow>::new(window.clone());
        let servo_box: Box<ServoAndWindowTrait> = Box::new(ServoAndWindow { servo: Some(servo), window: window });
        servo_box
    };

    // let url = ServoUrl::parse("https://google.com").unwrap();
    let url = ServoUrl::parse("file:///mnt/servo-embedding-example/background.html").unwrap();
    // let url = ServoUrl::parse("http://169.254.169.254/").unwrap();
    let (sender, receiver) = ipc::channel().unwrap();
    servo.handle_events(vec![WindowEvent::NewBrowser(url, sender)]);
    let browser_id = receiver.recv().unwrap();
    servo.handle_events(vec![WindowEvent::SelectBrowser(browser_id)]);

    println!("Sleeping 2s before closing…");
    std::thread::sleep(std::time::Duration::from_millis(6000));

    info!("repainting");
    servo.repaint();  // causes segfault
    let pixels: Vec<u8> = servo.window().screenshot();
    let mut png_bytes: Vec<u8> = vec![];
    {
        let encoder = PNGEncoder::new(&mut png_bytes);
        println!("pixels size: {}", pixels.len());
        encoder.encode(&*pixels, device_width, device_height, image::ColorType::RGBA(8)).expect("Could not encode png");
    }
    println!("Successfully encoded png {}", png_bytes.len());
    {
        let mut file = File::create("/tmp/screenshot.png").expect("could not create file");
        file.write(&*png_bytes).expect("Failed to write");
        file.flush().expect("failed to flush");
    }

    servo.handle_events(vec![WindowEvent::CloseBrowser(browser_id)]);
    println!("Sleeping another 2s to wait for errors from other threads…");
    servo.handle_events(vec![WindowEvent::Quit]);
    std::thread::sleep(std::time::Duration::from_millis(2000));
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
        ScaleFactor::new(self.highdpi_factor as f32)
    }

    fn framebuffer_size(&self) -> TypedSize2D<u32, DevicePixel> {
        // let (width, height) = self.glutin_window.get_inner_size().unwrap();
        // self.highdpi_factor() * width, self.highdpi_factor() * height
        TypedSize2D::new(self.device_width, self.device_height)
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
        TypedSize2D::new(self.logical_width as f32, self.logical_height as f32)
    }

    fn client_window(&self, _id: BrowserId) -> (Size2D<u32>, Point2D<i32>) {
        // let (width, height) = self.glutin_window.get_inner_size().unwrap();
        // let (x, y) = self.glutin_window.get_position().unwrap();
        let (x, y) = (0, 0);
        (Size2D::new(self.logical_width, self.logical_height), Point2D::new(x as i32, y as i32))
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
