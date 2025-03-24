use parse_size::parse_size;
use rand::{RngCore, SeedableRng, rngs::StdRng};
use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

pub fn create_random_file(dir: &Path, size: &str, seed: u64) -> anyhow::Result<PathBuf> {
    const PAGE_SIZE: u64 = 1024 * 4;
    let mut r = StdRng::seed_from_u64(seed);

    let num_page = {
        let num_bytes = parse_size(size)?;
        assert!(num_bytes % PAGE_SIZE == 0);
        num_bytes / PAGE_SIZE
    };
    let path = dir.join("hello");

    // write data
    {
        let mut file = File::create(&path)?;
        let mut buffer = [0; PAGE_SIZE as usize];
        for _ in 0..num_page {
            r.fill_bytes(&mut buffer);
            file.write_all(&buffer)?;
        }
        println!()
    }

    Ok(path)
}

#[cfg(test)]
mod tests {
    use std::{fs::File, os::unix::fs::MetadataExt};

    use tempfile::tempdir;

    use crate::create_random_file;

    #[test]
    fn test_create_random_file() -> anyhow::Result<()> {
        let dir = tempdir()?;
        let path = create_random_file(dir.path(), "128 MiB", 123456)?;
        let file = File::open(path)?;
        assert_eq!(file.metadata()?.size(), 128 * 1024 * 1024);

        Ok(())
    }
}
