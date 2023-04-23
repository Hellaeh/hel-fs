use std::{
	fs::{self, ReadDir},
	io,
	path::Path,
};

#[derive(Debug)]
pub struct ReadDirRec(Vec<ReadDir>);

/// Reads a directory recursively. Order is not guaranteed.
pub fn read_dir_rec<P: AsRef<Path>>(path: P) -> io::Result<ReadDirRec> {
	Ok(fs::read_dir(path)?.into())
}

impl From<ReadDir> for ReadDirRec {
	fn from(value: std::fs::ReadDir) -> Self {
		const CAP: usize = 5;
		let mut inner = Vec::with_capacity(CAP);
		inner.push(value);
		Self(inner)
	}
}

impl Iterator for ReadDirRec {
	type Item = fs::DirEntry;

	fn next(&mut self) -> Option<Self::Item> {
		let last = self.0.last_mut()?;

		let Some(Ok(entry)) = last.next() else {
			self.0.pop();
			return <Self as Iterator>::next(self)
		};

		let Ok(file_type) = entry.file_type() else {
			return <Self as Iterator>::next(self)
		};

		if !file_type.is_dir() {
			return Some(entry);
		};

		let Ok(dir) = fs::read_dir(entry.path()) else {
			return <Self as Iterator>::next(self);
		};

		self.0.push(dir);

		<Self as Iterator>::next(self)
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn read_dir() {
		let contents: Vec<_> = crate::read_dir_rec(".").unwrap().collect();

		assert!(!contents.is_empty());

		assert!(
			contents
				.iter()
				.filter(|entry| entry.file_name().to_str().unwrap().ends_with(".rs"))
				.count() > 0
		);
	}
}
