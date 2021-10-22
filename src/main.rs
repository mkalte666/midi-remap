// Disable the console on windows builds
#![windows_subsystem = "windows"]

mod mapping;
mod mapper;
mod errors;

use clap::{App, Arg, ErrorKind};
use midir::{MidiInput, MidiOutput, Ignore, MidiOutputConnection};
use midly::{live::LiveEvent, MidiMessage};

use crate::errors::{*};
use crate::mapper::Mapper;

fn print_available_ports() -> Result<(), Box<dyn std::error::Error>> {
    let mut midi_in = MidiInput::new("Input Listing")?;
    midi_in.ignore(Ignore::None);
    let midi_out = MidiOutput::new("Output Listing")?;

    eprintln!("Available input ports:");
    for (i, p) in midi_in.ports().iter().enumerate() {
        eprintln!("{}: \"{}\"", i, midi_in.port_name(p)?);
    }
    eprintln!("Available output ports:");
    for (i, p) in midi_out.ports().iter().enumerate() {
        eprintln!("{}: \"{}\"", i, midi_out.port_name(p)?);
    }

    Ok(())
}

fn print_midi_event(channel : u8, message : MidiMessage)
{
    eprint!("{}:", channel);
    match message {
        MidiMessage::NoteOff { key, vel } => { eprintln!("NoteOff:{}:{}", key, vel)}
        MidiMessage::NoteOn { key, vel } => { eprintln!("NoteOn:{}:{}", key, vel)}
        MidiMessage::Aftertouch { key, vel } => { eprintln!("Aftertouch:{}:{}", key, vel)}
        MidiMessage::Controller { controller, value } => {eprintln!("Control:{}:{}", controller, value)}
        MidiMessage::ProgramChange { program } => {eprintln!("ProgramChange:{}", program)}
        MidiMessage::ChannelAftertouch { vel } => {eprintln!("ChannelAftertouch:{}", vel)}
        MidiMessage::PitchBend { bend } => {eprintln!("PitchBend:{}", bend.as_f64())}
    }
}

fn remap_message(mapper: Mapper, conn_out: &mut MidiOutputConnection, event: midly::live::LiveEvent, enable_debug_prints : bool) {
    match event {
        LiveEvent::Midi { channel, message} => {
            if enable_debug_prints {
                eprint!("Got ");
                print_midi_event(channel.as_int(), message);
            }
            let new_message = mapper.map_message(message);
            let new_ev = LiveEvent::Midi {
                channel,
                message: new_message
            };
            let mut buf = Vec::new();
            new_ev.write(&mut buf).unwrap();
            match conn_out.send(&buf[..]) {
                Ok(_) => {}
                Err(e) => { eprintln!("Something went wrong while sending: {}", e)}
            }
            if enable_debug_prints {
                eprint!("Put ");
                print_midi_event(channel.as_int(), new_message);
            }
        }
        _ => ()
    }
}

fn start_app(args : &clap::ArgMatches, mapper : Mapper) -> Result<(), Box<dyn std::error::Error>> {

    let name_in = args.value_of("input").unwrap();
    let name_out = args.value_of("output").unwrap();
    let midi_in = MidiInput::new("midi-remap Input")?;
    let midi_out = MidiOutput::new("midi-remap Output")?;

    // we need to wait for something to tell us that the connection is borked
    let port_in = midi_in.ports().into_iter().find(|x| midi_in.port_name(x).unwrap() == name_in ).ok_or(PortOpenError::from_port(name_in))?;
    let port_out = midi_out.ports().into_iter().find(|x| midi_out.port_name(x).unwrap() == name_out ).ok_or(PortOpenError::from_port(name_out))?;

    let enable_debug_prints = args.is_present("debug");
    if enable_debug_prints {
        eprintln!("Debug printing enabled. Embrace the spam!");
    }
    let mut conn_out = midi_out.connect(&port_out, "midi-remap in conn")?;
    let _connection = midi_in.connect(&port_in,"midi-remap out conn", move |_, message, _| {
        let event = LiveEvent::parse(message).unwrap();
        remap_message(mapper.clone(), &mut conn_out, event, enable_debug_prints);
    }, ())?;

    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
        //check if input port still exists and abort if not
        let port_check_input = MidiInput::new("midi-remap in-Port Check")?;
        port_check_input.ports().into_iter().find(|x| port_check_input.port_name(x).unwrap() == name_in ).ok_or(PortLostError::from_port(name_in))?;
        let port_check_output = MidiOutput::new("midi-remap out-Port Check")?;
        port_check_output.ports().into_iter().find(|x| port_check_output.port_name(x).unwrap() == name_in ).ok_or(PortLostError::from_port(name_in))?;
    }
}

fn main() {
    let matches_or_error = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("Forward and map midi commands from one port to another")
        .arg(Arg::with_name("input")
            .short("i")
            .required(true)
            .help("Sets the input midi port.")
            .takes_value(true)
        )
        .arg(Arg::with_name("output")
            .short("o")
            .required(true)
            .help("Sets the output midi port.")
            .takes_value(true)

    )
        .arg(Arg::with_name("config")
            .short("c")
            .required(true)
            .help("Config file in json format")
            .takes_value(true)
    )
        .arg(Arg::with_name("debug")
            .short("d")
            .required(false)
            .help("Shows all incoming and outgoing midi notes")
            .takes_value(false)
        )
        .get_matches_safe();

    if matches_or_error.is_err() {
        let err = matches_or_error.err().unwrap();
        if err.kind != ErrorKind::VersionDisplayed && err.kind != ErrorKind::HelpDisplayed {
            match print_available_ports() {
                Ok(_) => (),
                Err(err) => eprintln!("Could not display available Ports: {}", err)
            }
        }
        err.exit();
    }



    let matches = matches_or_error.unwrap();
    match Mapper::new_from_json_file(matches.value_of("config").unwrap()) {
        Ok(mapper) => {
            loop {
                match start_app(&matches, mapper.clone()) {
                    Ok(_) => eprintln!("Unexpected exit of mapper. Restarting..."),
                    Err(err) => eprintln!("Unexpected exit of mapper: {}. Restarting...", err)
                }

                std::thread::sleep(std::time::Duration::from_secs(1));
            }
        }
        Err(error) => {eprintln!("Could not create mapper: {}", error)}
    }

}
