use parse_size::parse_size;
use rand::{Rng, RngCore, SeedableRng, rngs::StdRng};
use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};
use uuid::Uuid;

const ALIGN: u64 = 1024 * 4;

pub fn generate_random_file_offset(
    file_size: u64,
    block_size: u64,
    num_offsets: usize,
    seed: u64,
) -> Vec<u64> {
    let last = file_size - block_size;
    let mut r = StdRng::seed_from_u64(seed);

    let offset_candidates: Vec<_> = (0..=last).step_by(ALIGN as usize).collect();

    (0..num_offsets)
        .map(|_| r.random_range(0..offset_candidates.len()))
        .map(|index| (index as u64) * ALIGN)
        .collect()
}

pub fn create_random_file(dir: &Path, size: &str, seed: u64) -> anyhow::Result<PathBuf> {
    let mut r = StdRng::seed_from_u64(seed);

    let num_page = {
        let num_bytes = parse_size(size)?;
        assert!(num_bytes % ALIGN == 0);
        num_bytes / ALIGN
    };
    let path = dir.join(Uuid::new_v4().to_string());

    // write data
    {
        let mut file = File::create(&path)?;
        let mut buffer = [0; ALIGN as usize];
        for _ in 0..num_page {
            r.fill_bytes(&mut buffer);
            file.write_all(&buffer)?;
        }
    }

    Ok(path)
}

#[cfg(test)]
mod tests {
    use std::{fs::File, os::unix::fs::MetadataExt};

    use parse_size::parse_size;
    use tempfile::tempdir;

    use crate::create_random_file;
    use crate::generate_random_file_offset;

    #[test]
    fn test_generate_random_offsets() -> anyhow::Result<()> {
        let file_size = parse_size("128 KiB")?;

        let offsets = generate_random_file_offset(file_size, parse_size("8 KiB")?, 5, 123456);
        println!("offsets={:?}", offsets);

        Ok(())
    }

    #[test]
    fn test_create_random_file() -> anyhow::Result<()> {
        let dir = tempdir()?;
        let path = create_random_file(dir.path(), "128 MiB", 123456)?;
        let file = File::open(path)?;
        assert_eq!(file.metadata()?.size(), 128 * 1024 * 1024);

        Ok(())
    }
}
