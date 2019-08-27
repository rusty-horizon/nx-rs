use os;


pub struct Keyboard {
    kbd: ::libnx::SwkbdConfig,
}

pub enum KeyboardPreset {
    Default,
    Password,
    UserName,
    DownloadCode,
}

impl Keyboard {
    pub fn new() -> os::Result<Self> {
        unsafe {
            let mut kbd: ::libnx::SwkbdConfig = std::mem::zeroed();
            let rc = ::libnx::swkbdCreate(&mut kbd, 0);
            result_final!(rc, Self { kbd: kbd })
        }
    }

    pub fn set_preset(&mut self, preset: &KeyboardPreset) {
        unsafe {
            match preset {
                KeyboardPreset::Default => ::libnx::swkbdConfigMakePresetDefault(&mut self.kbd),
                KeyboardPreset::Password => ::libnx::swkbdConfigMakePresetPassword(&mut self.kbd),
                KeyboardPreset::UserName => ::libnx::swkbdConfigMakePresetUserName(&mut self.kbd),
                KeyboardPreset::DownloadCode => {
                    ::libnx::swkbdConfigMakePresetDownloadCode(&mut self.kbd)
                }
            };
        }
    }

    pub fn set_ok_button_text(&mut self, text: &str) {
        unsafe {
            ::libnx::swkbdConfigSetOkButtonText(&mut self.kbd, text.as_ptr());
        }
    }

    pub fn show(&mut self) -> os::Result<String> {
        unsafe {
            let mut out_buf: [u8; 500] = [0; 500];
            let out_ptr = out_buf.as_mut_ptr();
            let rc = ::libnx::swkbdShow(&mut self.kbd, out_ptr, 500);
            result_final!(
                rc,
                String::from_utf8_lossy(std::slice::from_raw_parts(out_ptr, 500)).to_string()
            )
        }
    }
}

impl Drop for Keyboard {
    fn drop(&mut self) {
        unsafe {
            ::libnx::swkbdClose(&mut self.kbd);
        }
    }
}
