use os;


handle!(0 in ::libnx::smInitialize(), ::libnx::smExit(), {
    pub fn get_service(&self, name: &str) -> os::Result<::libnx::Service> {
        unsafe {
            let mut srv: ::libnx::Service = std::mem::zeroed();
            let rc = ::libnx::smGetService(&mut srv, name.as_ptr());
            result_final!(rc, srv)
        }
    }
});
