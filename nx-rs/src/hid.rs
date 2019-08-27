

#[derive(Clone, Copy)]
pub enum Controller {
    Invalid,
    Auto,
    Handheld,
    Player(u8),
}

pub enum Key {
    None,
    A = 1,
    B = 2,
    X = 4,
    Y = 8,
    LStick = 16,
    RStick = 32,
    L = 64,
    R = 128,
    ZL = 256,
    ZR = 512,
    Plus = 1024,
    Minus = 2048,
    DPadRight = 16384,
    DPadUp = 8192,
    DPadDown = 32768,
    DPadLeft = 4096,
}

pub enum JoyConHoldMode {
    Default,
    Horizontal,
}

fn ctrlid_to_controller(id: ::libnx::HidControllerID) -> Controller {
    match id {
        ::libnx::HidControllerID_CONTROLLER_PLAYER_1 => Controller::Player(1),
        ::libnx::HidControllerID_CONTROLLER_PLAYER_2 => Controller::Player(2),
        ::libnx::HidControllerID_CONTROLLER_PLAYER_3 => Controller::Player(3),
        ::libnx::HidControllerID_CONTROLLER_PLAYER_4 => Controller::Player(4),
        ::libnx::HidControllerID_CONTROLLER_PLAYER_5 => Controller::Player(5),
        ::libnx::HidControllerID_CONTROLLER_PLAYER_6 => Controller::Player(6),
        ::libnx::HidControllerID_CONTROLLER_PLAYER_7 => Controller::Player(7),
        ::libnx::HidControllerID_CONTROLLER_PLAYER_8 => Controller::Player(8),
        ::libnx::HidControllerID_CONTROLLER_HANDHELD => Controller::Handheld,
        ::libnx::HidControllerID_CONTROLLER_P1_AUTO => Controller::Auto,
        _ => Controller::Invalid,
    }
}

fn controller_to_ctrlid(id: Controller) -> ::libnx::HidControllerID {
    match id {
        Controller::Player(1) => ::libnx::HidControllerID_CONTROLLER_PLAYER_1,
        Controller::Player(2) => ::libnx::HidControllerID_CONTROLLER_PLAYER_2,
        Controller::Player(3) => ::libnx::HidControllerID_CONTROLLER_PLAYER_3,
        Controller::Player(4) => ::libnx::HidControllerID_CONTROLLER_PLAYER_4,
        Controller::Player(5) => ::libnx::HidControllerID_CONTROLLER_PLAYER_5,
        Controller::Player(6) => ::libnx::HidControllerID_CONTROLLER_PLAYER_6,
        Controller::Player(7) => ::libnx::HidControllerID_CONTROLLER_PLAYER_7,
        Controller::Player(8) => ::libnx::HidControllerID_CONTROLLER_PLAYER_8,
        Controller::Handheld => ::libnx::HidControllerID_CONTROLLER_HANDHELD,
        Controller::Auto => ::libnx::HidControllerID_CONTROLLER_P1_AUTO,
        _ => ::libnx::HidControllerID_CONTROLLER_UNKNOWN,
    }
}

pub fn is_controller_connected(ctrl: Controller) -> bool {
    unsafe { ::libnx::hidIsControllerConnected(controller_to_ctrlid(ctrl)) }
}

pub fn flush() {
    unsafe {
        ::libnx::hidScanInput();
    }
}

pub fn input_down(ctrl: Controller) -> u64 {
    flush();
    unsafe {
        ::libnx::hidKeysDown(controller_to_ctrlid(ctrl))
    }
}

pub fn input_up(ctrl: Controller) -> u64 {
    unsafe {
        flush();
        ::libnx::hidKeysUp(controller_to_ctrlid(ctrl))
    }
}

pub fn input_held(ctrl: Controller) -> u64 {
    unsafe {
        flush();
        ::libnx::hidKeysHeld(controller_to_ctrlid(ctrl))
    }
}

pub fn get_touch_count() -> u32 {
    unsafe { ::libnx::hidTouchCount() }
}

pub fn get_touch_coords(index: u32) -> (u32, u32) {
    flush();
    unsafe {
        let mut tch: ::libnx::touchPosition = std::mem::zeroed();
        ::libnx::hidTouchRead(&mut tch, index);
        (tch.px, tch.py)
    }
}
