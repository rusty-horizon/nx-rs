

pub fn initialize() {
    unsafe {
        ::libnx::consoleInit(std::ptr::null_mut());
    }
}

pub fn exit() {
    unsafe {
        ::libnx::consoleExit(std::ptr::null_mut());
    }
}

pub fn clear() {
    unsafe {
        ::libnx::consoleClear();
    }
}

pub fn flush() {
    unsafe {
        ::libnx::consoleUpdate(std::ptr::null_mut());
    }
}