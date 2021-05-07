# vsthost
A simple VST host.

# About (note: this library is a big WIP)
The idea here is to provide a swiss army knife for VST hosting.<br>

- `start_layout_vsthost` to opens a VST using NFD to choose the DLL/lib
  * This returns the `PluginInstance` and `Editor`
  * note: The `PluginInstance` can be sent across threads, but not Editor.
- `open_window` provides a winit window given the editor which will attach itself to the window.
  * This returns the `Window` and `EventLoop`
  * The use of `EventLoop` will determine how you want the VST to run
      * In a DAW setting I would reccomend running GUI on the main thread, and calling `run_return` on that thread while sending your `PluginInstance` to the audio processing thread
