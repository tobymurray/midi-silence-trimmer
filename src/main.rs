use midly::{MidiMessage, Smf, TrackEventKind};
use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

fn trim_silence(input_path: &str, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
	// Read MIDI file
	let mut file = File::open(input_path)?;
	let mut buffer = Vec::new();
	file.read_to_end(&mut buffer)?;

	// Parse MIDI file
	let mut smf = Smf::parse(&buffer)?;

	if smf.tracks.len() != 1 {
		return Err(format!("Expected exactly 1 track, but found {}.", smf.tracks.len()).into());
	}

	let track = smf.tracks.first_mut().unwrap();

	// Find the position of the first note played
	let first_note = track.iter().position(|event| {
		if let TrackEventKind::Midi {
			message: MidiMessage::NoteOn { .. },
			..
		} = &event.kind
		{
			return true;
		}
		false
	});

	let mut empty_track = Vec::new();

	let trimmed_track = match first_note {
		Some(index) => {
			println!(
				"Found a note being played at: {} with delta {}",
				index, track[index].delta
			);
			track[index].delta = 0.into();
			track
		}
		None => {
			println!("Found no notes being played");
			&mut empty_track
		}
	};

	let trimmed_smf = Smf {
		header: smf.header,
		tracks: vec![trimmed_track.to_vec()],
	};

	// Write the trimmed MIDI file
	let mut output_buffer = Vec::new();
	trimmed_smf.write(&mut output_buffer)?;

	let mut output_file = File::create(output_path)?;
	output_file.write_all(&output_buffer)?;

	Ok(())
}

fn main() {
	let mut args = env::args().skip(1); // Skip the first argument (program name)

	// Get input and output paths from command-line arguments
	let input_midi_path = args.next().unwrap_or_else(|| {
		eprintln!(
			"Usage: {} <input_midi_path> [output_midi_path]",
			env::args().next().unwrap()
		);
		std::process::exit(1);
	});

	let output_midi_path = args.next().unwrap_or_else(|| default_output_filename(&input_midi_path));

	if let Err(err) = trim_silence(&input_midi_path, &output_midi_path) {
		eprintln!("Error: {}", err);
	} else {
		println!("MIDI file trimmed successfully!");
	}
}

fn default_output_filename(input_filename: &str) -> String {
	// Convert the input path to a PathBuf
	let input_path = Path::new(input_filename);
	let mut output_path = PathBuf::from(input_path);

	// Get the file stem (filename without extension)
	if let Some(file_stem) = input_path.file_stem() {
		// Append "-trimmed" to the file stem
		let trimmed_stem = format!("{}-trimmed", file_stem.to_string_lossy());

		// Replace the file stem in the output path
		output_path.set_file_name(trimmed_stem);
	}

	// Add the file extension to the output path
	if let Some(extension) = input_path.extension() {
		output_path.set_extension(extension);
	}

	output_path.to_string_lossy().into_owned()
}
