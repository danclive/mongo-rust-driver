use crate::sys::mongoc::*;
use crate::core::uri::Uri;
use crate::core::client::Client;

pub struct ClientPool(*mut mongoc_client_pool_t);

impl ClientPool {
    pub fn new(uri: &Uri) -> ClientPool {
        unsafe { mongoc_init(); }

        let pool = unsafe {
            mongoc_client_pool_new(uri.as_ptr())
        };

        assert!(!pool.is_null());

        unsafe { mongoc_client_pool_set_error_api(pool, 2); }

        ClientPool(pool)
    }

    pub fn from_ptr(ptr: *mut mongoc_client_pool_t) -> ClientPool {
        assert!(!ptr.is_null());
        ClientPool(ptr)
    }

    pub fn as_ptr(&self) -> *const mongoc_client_pool_t {
        assert!(!self.0.is_null());
        self.0
    }

    pub fn as_mut_ptr(&self) -> *mut mongoc_client_pool_t {
        assert!(!self.0.is_null());
        self.0
    }

    pub fn max_size(&self, size: u32) {
        assert!(!self.0.is_null());
        unsafe {
            mongoc_client_pool_max_size(self.0, size)
        }
    }

    pub fn min_size(&self, size: u32) {
        assert!(!self.0.is_null());
        unsafe {
            mongoc_client_pool_min_size(self.0, size)
        }
    }

    pub fn pop(&self) -> Client {
        assert!(!self.0.is_null());
        let client = unsafe {
            mongoc_client_pool_pop(self.0)
        };

        Client::from_ptr(client)
    }

    pub fn push(&self, client: &Client) {
        assert!(!self.0.is_null());
        let ptr = client.as_mut_ptr();

        unsafe { mongoc_client_pool_push(self.0, ptr) }
    }

    pub fn try_pop(&self) -> Option<Client> {
        assert!(!self.0.is_null());
        let client = unsafe {
            mongoc_client_pool_try_pop(self.0)
        };

        if client.is_null() {
            None
        } else {
            Some(Client::from_ptr(client))
        }
    }
}

unsafe impl Send for ClientPool {}
unsafe impl Sync for ClientPool {}

impl Drop for ClientPool {
    fn drop(&mut self) {
        assert!(!self.0.is_null());
        unsafe {
            mongoc_client_pool_destroy(self.0);
            mongoc_cleanup();
        }
    }
}
