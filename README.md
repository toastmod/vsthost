# vsthost
A simple VST host toolset.

# About (note: this library is to be updated as I work on my DAW)
The idea here is to provide a swiss army knife for VST hosting.<br>

- `create_vsthost` to opens a VST given the path to the DLL/lib
  * The `nfd-select` feature enables `Method::FileSelectDialog` which will open the NFD to fetch a path.
  * This returns the `PluginInstance` and `Editor`
  * note: The `PluginInstance` can be sent across threads, but not Editor.
- `open_window` provides a winit window given the editor which will attach itself to the window.
  * This returns the `Window` and `EventLoop`
  * The use of `EventLoop` will determine how you want the VST to run
      * In a DAW setting I would recommend running GUI on the main thread, and calling `run_return` on that thread while sending your `PluginInstance` to the audio processing thread

# TODOs
- Clean up the code
- Add support for bridging (32bit<->64bit)
- Possibly expand on windowing?
