use std::{
	collections::{HashMap, HashSet},
	fs,
};

use proc_macro2::Span;
use quote::ToTokens;
use syn::{spanned::Spanned, File, Item, ItemMod};

fn main() {
	let common_struct_file_path = "./oasbi/src/common/generated.rs";
	let match_and_remove_struct_file_paths = [
//		"./oasbi/src/amf.rs",
//		"./oasbi/src/ausf.rs",
//		"./oasbi/src/chf.rs",
//		"./oasbi/src/lib.rs",
//		"./oasbi/src/nrf.rs",
//		"./oasbi/src/nssf.rs",
//		"./oasbi/src/pcf.rs",
//		"./oasbi/src/smf.rs",
//		"./oasbi/src/udm.rs",
//		"./oasbi/src/udr.rs",
//		"./oasbi/src/upf.rs",
	];
	for file_path in match_and_remove_struct_file_paths {
		println!("Parsing Currently: {}", file_path);
		match process_files(common_struct_file_path, file_path) {
			Ok(_) => println!("Processing completed successfully."),
			Err(e) => eprintln!("Error: {}", e),
		}
	}
}

fn process_files(
	common_struct_file_path: &str,
	file_to_remove_common_structs_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
	// Read both files
	let common_struct_content = fs::read_to_string(common_struct_file_path)?;
	let file_to_remove_common_structs_content =
		fs::read_to_string(file_to_remove_common_structs_path)?;

	// Parse both files
	let common_struct_parsed_file: File = syn::parse_file(&common_struct_content)?;
	let file_to_remove_common_structs_parsed_file: File =
		syn::parse_file(&file_to_remove_common_structs_content)?;

	// Extract item spans from `generated.rs`
	let generated_spans = extract_item_spans(&common_struct_parsed_file.items);

	// Extract item spans from `amf.rs`
	let amf_spans = extract_item_spans(&file_to_remove_common_structs_parsed_file.items);

	// Collect lines to remove from `amf.rs`
	let lines_to_remove: HashSet<usize> = amf_spans
		.iter()
		.filter(|(name, _)| generated_spans.contains_key(name.to_owned()))
		.flat_map(|(_, range)| range.clone())
		.collect();

	println!("Number of lines to remove: {}", lines_to_remove.len());
	// Remove matching lines from `amf.rs`
	let filtered_file_to_remove_common_structs_content =
		remove_lines(&file_to_remove_common_structs_content, &lines_to_remove);

	// Save the modified content to a new file
	fs::write(
		file_to_remove_common_structs_path,
		&filtered_file_to_remove_common_structs_content,
	)?;
	Ok(())
}

// Extracts item spans (names and their line ranges) from a parsed file
fn extract_item_spans(items: &Vec<Item>) -> HashMap<String, std::ops::Range<usize>> {
	let mut item_spans = HashMap::new();

	for item in items {
		let (name, span) = match item {
			Item::Struct(item_struct) => (
				item_struct.to_owned().into_token_stream(),
				item_struct.span(),
			),
			Item::Enum(item_enum) => (item_enum.to_owned().into_token_stream(), item_enum.span()),
			Item::Impl(item_impl) => (item_impl.to_owned().into_token_stream(), item_impl.span()),
			Item::Mod(ItemMod {
				content: Some((_, mod_items)),
				..
			}) => {
				let mod_item_spans = extract_item_spans(mod_items);
				item_spans.extend(mod_item_spans);
				continue;
			}
			_ => continue,
		};

		let range = get_line_range(span);
		item_spans.insert(name.to_string(), range);
	}

	item_spans
}

// Converts a span to a line range in the file content
fn get_line_range(span: Span) -> std::ops::Range<usize> {
	let start_line = span.start().line;
	let end_line = span.end().line;
	start_line..end_line + 1
}

// Removes specified lines from the file content
fn remove_lines(
	file_content: &str,
	lines_to_remove: &HashSet<usize>,
) -> String {
	file_content
		.lines()
		.enumerate()
		.filter(|(i, _)| !lines_to_remove.contains(&(i + 1))) // Line numbers are 1-based
		.map(|(_, line)| line)
		.collect::<Vec<_>>()
		.join("\n")
}
