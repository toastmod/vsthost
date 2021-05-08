//use tuix::Application;
use std::sync::Arc;
use raw_window_handle::{RawWindowHandle, HasRawWindowHandle};
use vst::editor::Editor;
pub use winit::window::{WindowBuilder, Window};
pub use winit::event_loop::{ControlFlow, EventLoop};
pub use winit::event::{Event, WindowEvent};
pub use winit::platform::run_return::EventLoopExtRunReturn;

pub fn open_window(size: (i32, i32), title: String, editor: &mut Box<Editor>) -> (Window, EventLoop<()>) {

    let mut event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title(title.as_str())
        .with_inner_size(winit::dpi::LogicalSize::new(size.0, size.1))
        // .with_decorations(false)
        .build(&event_loop)
        .unwrap();


    let handle = window.raw_window_handle();
    match handle {

        #[cfg(target_os = "linux")]
        RawWindowHandle::Xcb(w) => {
            println!("Xcb");
            editor.open(w.window as *mut _);
        }
        #[cfg(target_os = "linux")]
        RawWindowHandle::Xlib(w) => {
            println!("Xlib");
            editor.open(w.window as u32 as *mut _);
        }

        #[cfg(target_os = "linux")]
        RawWindowHandle::Wayland(w) => println!("Window handle: {:?}", w),

        #[cfg(target_os = "windows")]
        RawWindowHandle::Windows(hwnd)=>{
          editor.open(hwnd.hwnd as *mut _);
        },

        #[cfg(target_os = "macos")]
        RawWindowHandle::MacOS(w) => {
            editor.open(w.window as u32 as *mut _);
        }

        _ => {
            println!("Unsupported platform");
            //return;
        }


    }

    // run event loop

    // loop {
    //     println!("new run ret");
    //     event_loop.run_return( |event, _, control_flow| {
    //         *control_flow = ControlFlow::Exit;
    //         //println!("{:?}", event);
    //         // match event {
    //         //     Event::NewEvents(_) => {}
    //         //     Event::WindowEvent { .. } => {}
    //         //     Event::DeviceEvent { .. } => {}
    //         //     Event::UserEvent(_) => {}
    //         //     Event::Suspended => {}
    //         //     Event::Resumed => {}
    //         //     Event::MainEventsCleared => {}
    //         //     Event::RedrawRequested(_) => {}
    //         //     Event::RedrawEventsCleared => {}
    //         //     Event::LoopDestroyed => {}
    //         // }
    //     });
    // }

    print!("giving eventloop");

    (window, event_loop)
}

