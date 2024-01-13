# MIDI Silence Trimmer

A Rust program to trim silence in MIDI files (by modifying the delta time of the first note and end of track event).

## Overview

This program:

- reads a MIDI file
- identifies the first played note in the track
- identifies the end of track event
- trims the silence by setting the delta time of the note and end of track event to 0
- writes out the modified MIDI file to the specified output path

## Usage

```sh
cargo run -- <input_midi_path> [output_midi_path]

    <input_midi_path>: Path to the input MIDI file.
    [output_midi_path]: (Optional) Path to the output MIDI file. If not provided, a default output filename will be generated.
```

Example

```sh
cargo run -- input.mid output.mid
```

This command trims the silence at the beginning of input.mid file and saves the modified MIDI file as output.mid. input.mid is not modified.

## Dependencies

- [midly](https://github.com/kovaxis/midly): a feature-complete MIDI decoder and encoder designed for efficiency and ease of use.
