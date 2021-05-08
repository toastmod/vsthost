use crate::{PluginInstance, Window, Method, create_vsthost, Plugin};
use crate::winit_window::{EventLoop, open_window, EventLoopExtRunReturn, ControlFlow};

pub struct AudioFrame {
    instance: PluginInstance,
    window: Window,
    event_loop: EventLoop<()>
}

impl AudioFrame {
    fn new() -> Self {
        let mut i = create_vsthost(Method::FileSelectDialog).unwrap();
        let mut editor = i.get_editor().unwrap();
        let (mut w, mut e) = open_window(editor.size(), String::new(), &mut editor);
        AudioFrame {
            instance: i,
            window: w,
            event_loop: e
        }
    }


    fn update_window(&mut self) {

        // handle events
        self.event_loop.run_return( |event, _, control_flow| {
            *control_flow = ControlFlow::Exit;
            //println!("{:?}", event);
            // match event {
            //     Event::NewEvents(_) => {}
            //     Event::WindowEvent { .. } => {}
            //     Event::DeviceEvent { .. } => {}
            //     Event::UserEvent(_) => {}
            //     Event::Suspended => {}
            //     Event::Resumed => {}
            //     Event::MainEventsCleared => {}
            //     Event::RedrawRequested(_) => {}
            //     Event::RedrawEventsCleared => {}
            //     Event::LoopDestroyed => {}
            // }
        });
        // render frame
        self.window.request_redraw();
    }

}