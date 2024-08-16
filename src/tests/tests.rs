#![cfg(test)]

use crate::twiddler_config::{txt_to_cfg, cfg_to_txt};

struct TempFile {
    path: String,
}

const CFG_PATH: &str = "src/tests/demo_cfg_dedupe.cfg";
const CFG_PATH_NO_DEDUPE: &str = "src/tests/demo_cfg_nodedupe.cfg";
const TXT_PATH: &str = "src/tests/demo_txt.txt";


impl TempFile {
    fn new(filename: String) -> Self {
        TempFile { path: filename }
    }

    fn path(&self) -> &str {
        &self.path
    }
}

impl Drop for TempFile {
    fn drop(&mut self) {
        let path = std::path::Path::new(&self.path);
        if path.exists() {
            std::fs::remove_file(path).unwrap();
        }
    }
}

// check that converting text to config and back produces the same result
#[test]
fn test_txt_to_cfg() {
    let new_cfg_path = TempFile::new(txt_to_cfg(TXT_PATH).unwrap());
    let new_cfg = std::fs::read(new_cfg_path.path()).unwrap();
    let original_cfg = std::fs::read(CFG_PATH).unwrap();
    assert_eq!(new_cfg, original_cfg);
}

#[test]
fn test_cfg_to_txt() {
    let new_txt_path = TempFile::new(cfg_to_txt(CFG_PATH).unwrap());
    let new_txt = std::fs::read_to_string(new_txt_path.path()).unwrap();
    let original_txt = std::fs::read_to_string(TXT_PATH).unwrap();
    assert_eq!(new_txt, original_txt);
}

#[test]
fn test_nodedupe_cfg_to_txt() {
    let new_txt_path = TempFile::new(cfg_to_txt(CFG_PATH_NO_DEDUPE).unwrap());
    let new_txt = std::fs::read_to_string(new_txt_path.path()).unwrap();
    let original_txt = std::fs::read_to_string(TXT_PATH).unwrap();
    assert_eq!(new_txt, original_txt);
}
