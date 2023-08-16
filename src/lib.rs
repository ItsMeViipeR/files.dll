use std::{ffi::{CStr, CString, c_char}, fs::File, io::{Read, Write}};

#[no_mangle]
pub extern "C" fn read_file(path: *const c_char) -> *mut c_char {
    let path = unsafe { CStr::from_ptr(path) };
    let path = path.to_str().unwrap();
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let c_str = CString::new(contents).unwrap();
    c_str.into_raw()
}

#[no_mangle]
pub extern "C" fn write_file(path: *const c_char, text: *const c_char) {
    let path_str = unsafe { CString::from_raw(path as *mut c_char) };
    let text_str = unsafe { CString::from_raw(text as *mut c_char) };

    let path_converted = path_str.to_str().unwrap();
    let text = text_str.to_str().unwrap();

    if File::open(path_converted).is_ok() {
        let content = (unsafe { CStr::from_ptr(read_file(path)) }).to_str().unwrap();
        println!("content: {}", content);

        let to_write = format!("{}{}", content, text);

        println!("to_write: {}", to_write);

        // open the file in read-write mod and write to_write into it
        let _ = File::create(path_converted).and_then(|mut file| file.write_all(to_write.as_bytes()));
    } else {
        let _ = File::create(path_converted).and_then(|mut file| file.write_all(text.as_bytes()));
    }

    println!("write_file: {}", path_converted);
}