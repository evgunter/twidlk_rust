#![cfg(test)]

use crate::{twiddler_config::{cfg_to_txt, text_to_usb, txt_to_cfg, usb_hid_to_text}, ChordOutput::SingleChord};

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

#[test]
fn raw_key_code() {
    let (mods, key_code) = match text_to_usb("<keycode 0x3F>".to_owned()) {
        Ok(SingleChord {modifier, key_code}) => (modifier, key_code),
        Err(v) => {
            assert!(false, "{:?}", v);
            (0x00, 0)  // to make it type check
        },
        Ok(v) => {
            assert!(false, "{:?}", v);
            (0x00, 0)  // to make it type check
        },
    };
    assert_eq!(mods, 0x00);
    assert_eq!(key_code, 0x3F);

    let (mods, key_code) = match text_to_usb("<space>".to_owned()) {
        Ok(SingleChord {modifier, key_code}) => (modifier, key_code),
        Err(v) => {
            assert!(false, "{:?}", v);
            (0x00, 0)  // to make it type check
        },
        Ok(v) => {
            assert!(false, "{:?}", v);
            (0x00, 0)  // to make it type check
        },
    };
    assert_eq!(mods, 0x00);
    assert_eq!(key_code, 0x2C);

    let (shifted, txt) = usb_hid_to_text(true, 0x00);  // key code not in USB_HID_TABLE
    assert_eq!(shifted, true);
    assert_eq!(txt, "<keycode 0x00>");
    let (shifted, txt) = usb_hid_to_text(false, 0xFF);  // key code not in USB_HID_TABLE
    assert_eq!(shifted, false);
    assert_eq!(txt, "<keycode 0xFF>");

    let (shifted, txt) = usb_hid_to_text(true, 0x2C);
    assert_eq!(shifted, true);
    assert_eq!(txt, "<space>");

    let (_, txt) = usb_hid_to_text(true, 0x04);
    assert_eq!(txt, "A");

    let (_, txt) = usb_hid_to_text(false, 0x1E);
    assert_eq!(txt, "1");

}
