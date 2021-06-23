//from https://stackoverflow.com/a/54817755
use std::env;
use std::io;
use std::path::{PathBuf, Path};

use path_clean::PathClean;

// use std::error;

// Change the alias to `Box<error::Error>`.
// type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

pub fn absolute_path(path: impl AsRef<Path>) -> io::Result<PathBuf> {
    let path = path.as_ref();

    let absolute_path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        env::current_dir()?.join(path)
    }.clean();

    Ok(absolute_path)
}

#[cfg(test)]
pub mod testing {
    use super::*;
    use data_encoding::HEXUPPER;
    use ring::digest::{Context, Digest, SHA256};
    use std::fs::File;
    use std::io::{BufReader, Read};
    // code from https://rust-lang-nursery.github.io/rust-cookbook/cryptography/hashing.html
    #[allow(dead_code)]
    fn sha256_digest<R: Read>(mut reader: R) -> io::Result<Digest> {
        let mut context = Context::new(&SHA256);
        let mut buffer = [0; 1024];

        loop {
            let count = reader.read(&mut buffer)?;
            if count == 0 {
                break;
            }
            context.update(&buffer[..count]);
        }

        Ok(context.finish())
    }

    #[allow(dead_code)]
    pub fn hash_file(path: &Path) -> io::Result<String> {
        let input = File::open(path)?;
        let reader = BufReader::new(input);
        let digest = sha256_digest(reader)?;

        Ok(HEXUPPER.encode(digest.as_ref()))
    }
}

// takes in (x, y),   (r, g, b, a)
