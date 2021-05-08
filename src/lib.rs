use std::{
    path::Path,
    sync::{
        mpsc::{channel, Sender},
        Arc, Mutex,
    },
};

use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
//use tuix::*;

use nfd::Response;
pub use vst::{
    host::{Host, PluginInstance, PluginLoadError, PluginLoader},
    plugin::Plugin,
};
pub use winit::window::Window;
use std::ffi::c_void;
use std::sync::atomic::AtomicU32;
use winit::dpi::{LogicalSize, PhysicalSize};
pub use ringbuf::*;
use vst::api;
use vst::event;
use crate::event_gen::{new_midievent, encode_midi_message_as_events};
use std::time::Duration;
use vst::buffer::AudioBuffer;
use vst::host::HostBuffer;
//use crate::newwindow::open_window;
pub use vst::editor::Editor;

mod host;
mod event_gen;
pub mod window;
pub mod newwindow;
pub mod winit_window;
mod audioframe;

#[derive(Default)]
struct VstHost;

impl Host for VstHost {}

#[derive(Clone, Debug, PartialEq)]
enum HostWidgetEvent {
    OpenFile,
}

#[derive(Default)]
pub struct HostWidget {
    host: Arc<Mutex<VstHost>>,
}

// impl BuildHandler for HostWidget {
//     type Ret = Entity;
//     fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
//         state.insert_stylesheet("src/theme.css");
//
//         let container = VBox::new().build(state, entity, |builder| builder);
//
//         let open_file_button = Button::with_label("Open File")
//             .on_press(Event::new(HostWidgetEvent::OpenFile).target(entity))
//             .build(state, container, |builder| {
//                 builder.class("open_file_dialogue_button")
//             });
//
//         self.label = Label::new("<Open a plugin first>").build(state, container, |builder| builder);
//
//         entity
//     }
// }
//
// impl EventHandler for HostWidget {
//     fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) -> bool {
//         if let Some(host_widget_event) = event.message.downcast::<HostWidgetEvent>() {
//             match host_widget_event {
//                 HostWidgetEvent::OpenFile => {
//                     if event.target == entity {
//                         let result = nfd::open_file_dialog(None, None).unwrap_or_else(|e| {
//                             panic!(e);
//                         });
//
//                         let path = match result {
//                             Response::Okay(file_path) => file_path,
//                             Response::OkayMultiple(mut files) => {
//                                 files.drain(..).take(1).next().unwrap()
//                             }
//                             _ => return false,
//                         };
//                         let mut instance = load(self.host.clone(), path).unwrap();
//                         if let Some(editor) = instance.get_editor() {
//                             let parent = todo!();
//                             //let success = editor.open(parent);
//                             println!("Editor window opened successfully");
//                         }
//
//                         self.label
//                             .set_text(state, &format!("{:?}", instance.get_info()));
//
//                         return true;
//                     }
//                 }
//             }
//         }
//
//         false
//     }
// }

fn load<H: Host, P: AsRef<Path>>(
    host: Arc<Mutex<H>>,
    path: P,
) -> Result<PluginInstance, PluginLoadError> {
    let mut loader = PluginLoader::load(path.as_ref(), Arc::clone(&host))?;
    loader.instance()
}

pub enum Method {
    Path(String),
    #[cfg(feature = "nfd-select")]
    FileSelectDialog
}

// pub struct VSTSeat{
//     host: Arc<Mutex<VstHost>>,
//     instance: PluginInstance
// }

//-> (Producer<[u8;3]>, Consumer<f32>, Application)
pub fn create_vsthost(method: Method) -> Result<PluginInstance, PluginLoadError> {

    // LOAD PLUGIN
    let host = Arc::new(Mutex::new(VstHost));

    let path: String = match method {
        Method::Path(p) => p,

        #[cfg(feature = "nfd-select")]
        Method::FileSelectDialog => {
            match nfd::open_file_dialog(None,None) {
                Ok(r) => {
                    match r {
                        Response::Okay(f) => {
                            f
                        }
                        Response::OkayMultiple(fs) => {
                            panic!("Too many picked!");
                        }
                        Response::Cancel => {
                            panic!("Nothing picked!");
                        }
                    }
                }
                Err(e) => {
                    panic!("{:?}",e);
                }
            }
        }
    };

    println!("loading VST: {:?}",path);
    load(host, &path)
}

// pub fn start_generator_vsthost() -> (Producer<[u8;3]>, Consumer<f32>, Box<Editor>) {
//
//     // LOAD PLUGIN
//
//     let args = std::env::args().collect::<Vec<_>>();
//     let host = Arc::new(Mutex::new(VstHost));
//
//     let lol = match nfd::open_file_dialog(None,None) {
//         Ok(r) => {
//             match r {
//                 Response::Okay(f) => {
//                     f
//                 }
//                 Response::OkayMultiple(fs) => {
//                     panic!("Too many picked!");
//                 }
//                 Response::Cancel => {
//                     panic!("Nothing picked!");
//                 }
//             }
//         }
//         Err(e) => {
//             panic!("{:?}",e);
//         }
//     };
//     println!("{:?}",lol);
//     let mut instance = load(host, &lol).unwrap();
//     let mut editor = instance.get_editor().unwrap();
//
//     println!("running application");
//
//     let mut midi_io: RingBuffer<[u8; 3]> = RingBuffer::new(1);
//     let mut sound_io: RingBuffer<f32> = RingBuffer::new(4096);
//
//     let (mut midi_to_plug, mut midi_from_player) = midi_io.split();
//     let (mut audio_to_player, mut audio_from_plug) = sound_io.split();
//
//     std::thread::spawn(move ||{
//         println!("VST midi/audio processing thread...");
//
//         //instance.resume();
//
//         // instance.set_block_size(1);
//         // instance.set_sample_rate(44100.0);
//
//         let mut count = 0;
//         loop {
//             let mut host_buffer: HostBuffer<f32> = HostBuffer::new(2, 2);
//             let inputs = vec![vec![0.0; 1]; 2];
//             let mut outputs = vec![vec![0.0; 1]; 2];
//             let mut audio_buffer = host_buffer.bind(&inputs, &mut outputs);
//
//             //print!("midi");
//
//             // count += 1;
//             // println!("{}",count);
//             // if count >= 44 {
//             //     println!("sending midi...");
//             //
//             //     count = 0;
//             // }
//
//             if !midi_from_player.is_empty() {
//                 let mut e = encode_midi_message_as_events(midi_from_player.pop().unwrap());
//                 instance.process_events(&e.events);
//             }
//
//             instance.process(&mut audio_buffer);
//             while audio_to_player.is_full() {
//                 // wait
//             }
//             audio_to_player.push(audio_buffer.split().1.get(0)[0]);
//
//         }
//     });
//     (midi_to_plug, audio_from_plug, editor)
// }

