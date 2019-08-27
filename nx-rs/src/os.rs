

pub type Result<T> = std::result::Result<T, u32>;

pub fn get_current_thread_handle() -> u32 {
    0xffff_8000
}

pub fn get_current_process_handle() -> u32 {
    0xffff_8001
}

pub fn is_nso() -> bool {
    unsafe { ::libnx::envIsNso() }
}

pub fn is_nro() -> bool {
    !is_nso()
}

pub fn env_exec_nro(path: &str, args: &str) {
    unsafe {
        ::libnx::envSetNextLoad(path.as_ptr(), args.as_ptr());
    }
    std::process::exit(0);
}

pub struct Version {
    pub ver: ::libnx::SetSysFirmwareVersion
}

impl std::default::Default for Version {
    fn default() -> Self {
        Version {
            ver: unsafe { std::mem::zeroed() }
        }
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.get())
    }
}

impl std::fmt::Debug for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", String::from(std::str::from_utf8(&self.ver.display_title).unwrap()))
    }
}

impl Version {
    fn get(&self) -> String {
        String::from(std::str::from_utf8(&self.ver.display_version).unwrap())
    }
}

pub fn get_version() -> Result<Version> {
    unsafe {
        let mut rc = ::libnx::setsysInitialize();
        result_assert!(rc);
        let mut ver = Version::default();
        rc = ::libnx::setsysGetFirmwareVersion(&mut ver.ver);
        result_final!(rc, ver)
    }
}