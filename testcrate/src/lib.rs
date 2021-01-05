extern "C" {
    pub fn yr_initialize() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn yr_finalize() -> ::std::os::raw::c_int;
}

#[cfg(test)]
mod tests {
    use crate::{yr_finalize, yr_initialize};

    #[test]
    fn yr_initialize_works() {
        assert_eq!(unsafe { yr_initialize() }, 0);
        assert_eq!(unsafe { yr_finalize() }, 0);
    }
}
