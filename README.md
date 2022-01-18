# Midi Remap
Input some midi, output (sometimes different) midi

This tool can help if you have devices that only send control commands, but you need Notes, like on GrandMA OnPc, or even just for generic midi-mapping-purposes. 

### Usage 
You can build and run this with cargo 

    cargo run 
    
This will print the list of available midi inputs and output. You can then run this with a config file 

    cargo run -- -i "a2jmidid:port 128:0" -o "a2jmidid:port 128:0" -c "some_config.json"
    
### Creating Configs 
First, an example: https://github.com/mkalte666/midi-remap/blob/main/examples/apc_mini_ma2.json

Configs are just json files of the following format: 
    
    Root-Object -> mappings -> Array of map objects 
    
The map objects have the following fields 

* `from` the midi command type to map from. This can be one of 
    
    pub enum MsgType {
        NoteOn,
        NoteOff,
        Aftertouch,
        Control,
    }
    
* `to` same values as `from`, but this is the target command type.
* `start` the id of the message to start from. These can be `0..127`, inclusive.
* `stop` the id where to stop mapping. These can be `0..127`, inclusive.
* `to_start` shifts the range.  

The final id for the new command is calulcated like this: 

    new_id = to_start + old_id-start

Example: `start = 10`, `stop=20`, `to_start=100`. If we get an argument of 15, the result will be 105. 
