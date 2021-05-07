use vst::api;
use vst::api::{MidiEvent, EventType, Event, Events};
use core::mem;

pub fn new_midievent(live: i32, note_len: i32, note_offset: i32, midi_data: [u8; 3], detune: i8, note_off_vel: u8) -> api::Event {

    let _note_len = note_len.to_ne_bytes();
    let _note_offset = note_offset.to_ne_bytes();
    println!("{}",std::mem::size_of::<MidiEvent>() as i32);

    api::Event {
        event_type: EventType::Midi,
        byte_size: std::mem::size_of::<MidiEvent>() as i32,
        delta_frames: 0,
        _flags: live,
        _reserved: [
            _note_len[0],
            _note_len[1],
            _note_len[2],
            _note_len[3],
            _note_offset[0],
            _note_offset[1],
            _note_offset[2],
            _note_offset[3],
            midi_data[0],
            midi_data[1],
            midi_data[2],
            0u8,
            detune.to_ne_bytes()[0],
            note_off_vel,
            0u8,
            0u8
        ]
    }
}

pub struct EventContainer {
    pub stored_event: Box<Event>,
    pub events: api::Events,
}

pub fn
encode_midi_message_as_events(message: [u8; 3]) -> EventContainer {
    let midi_event: MidiEvent = MidiEvent {
        event_type: EventType::Midi,
        byte_size: mem::size_of::<MidiEvent>() as i32,
        delta_frames: 0,
        flags: 1,
        note_length: 100,
        note_offset: 0,
        midi_data: [message[0], message[1], message[2]],
        _midi_reserved: 0,
        detune: 0,
        note_off_velocity: 0,
        _reserved1: 0,
        _reserved2: 0,
    };
    let mut event: Event = unsafe { std::mem::transmute(midi_event) };
    event.event_type = EventType::Midi;

    let events = Events {
        num_events: 1,
        _reserved: 0,
        events: [&mut event, &mut event], // Second one is a dummy
    };
    let mut ec = EventContainer {
        stored_event: Box::new(event),
        events,
    };
    ec.events.events[0] = &mut *(ec.stored_event); // Overwrite ptrs, since we moved the event into ec
    ec
}