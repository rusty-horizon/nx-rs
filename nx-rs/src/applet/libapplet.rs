use os;


pub struct LibraryApplet {
    holder: ::libnx::AppletHolder,
}

impl LibraryApplet {
    pub fn new(id: ::libnx::AppletId, mode: ::libnx::LibAppletMode, version: u32) -> os::Result<Self> {
        unsafe {
            let mut aph: ::libnx::AppletHolder = std::mem::zeroed();
            let mut rc = ::libnx::appletCreateLibraryApplet(&mut aph, id, mode);
            result_assert!(rc, {
                println!("Exit");
            });
            let mut largs: ::libnx::LibAppletArgs = std::mem::zeroed();
            ::libnx::libappletArgsCreate(&mut largs, version);
            rc = ::libnx::libappletArgsPush(&mut largs, &mut aph);
            result_final!(rc, Self { holder: aph })
        }
    }

    pub fn push_data(&mut self, data: *const u8, size: usize) -> os::Result<()> {
        unsafe {
            let rc =
                ::libnx::libappletPushInData(&mut self.holder, data as *const std::ffi::c_void, size);
            result_final!(rc)
        }
    }

    pub fn show(&mut self) -> os::Result<()> {
        unsafe {
            let rc = ::libnx::appletHolderStart(&mut self.holder);
            result_final!(rc)
        }
    }

    pub fn show_and_wait(&mut self) -> os::Result<()> {
        unsafe {
            let rc = ::libnx::appletHolderStart(&mut self.holder);
            result_assert!(rc);
            while ::libnx::appletHolderWaitInteractiveOut(&mut self.holder) {}
            ::libnx::appletHolderJoin(&mut self.holder);
            result_final!(rc)
        }
    }

    pub fn pop_data(&mut self, out: *mut u8, size: usize) -> os::Result<usize> {
        unsafe {
            let mut out_size: usize = 0;
            let rc = ::libnx::libappletPopOutData(
                &mut self.holder,
                out as *mut std::ffi::c_void,
                size,
                &mut out_size,
            );
            result_final!(rc, out_size)
        }
    }
}

impl Drop for LibraryApplet {
    fn drop(&mut self) {
        unsafe {
            ::libnx::appletHolderClose(&mut self.holder);
        }
    }
}
