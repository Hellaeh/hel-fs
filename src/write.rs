/// Same as `std::fs::write`, except writes to a
/// to temp buffer file, then renames it, if write was successful
pub fn write_safe<P, C>(path: P, contents: C) -> std::io::Result<()>
where
	P: AsRef<std::path::Path>,
	C: AsRef<[u8]>,
{
	let path = path.as_ref();
	let mut temp_path = path.to_path_buf();

	if !temp_path.set_extension("tmp") {
		return std::fs::write(path, contents);
	}

	if std::fs::write(&temp_path, contents).is_ok() {
		std::fs::rename(temp_path, path)?;
	}

	Ok(())
}

#[cfg(test)]
mod tests {
	use super::write_safe as ws;

	#[test]
	fn write_safe() {
		const TEST_FILE: &str = "test.tmp";
		const TEST_CONTENTS: &str = "Hello 12345";

		fn cleanup() -> bool {
			std::fs::remove_file(TEST_FILE).is_ok()
		}

		cleanup();

		ws(TEST_FILE, TEST_CONTENTS).unwrap();

		assert!(cleanup())
	}
}
