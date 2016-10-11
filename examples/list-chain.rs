extern crate libmnl_sys;
extern crate libnftnl_sys;
extern crate libc;

use std::ffi::{CStr, CString};
use std::ptr;
use std::os::raw::*;

use libmnl_sys::*;
use libnftnl_sys::*;

fn main() {
    unsafe {

        let mut buf = [0i8; socket::BUFFER_SIZE];
        let family = chain::NFPROTO::INET;

        let chain = chain::alloc();
        assert!(chain != ptr::null_mut());
        let seq = 0;
        let header = common::nlmsg_build_hdr(buf.as_mut_ptr(),
                                             nf_tables::nf_tables_msg_types::NFT_MSG_GETCHAIN as u16,
                                             family as u16,
                                             4,
                                             seq);
        let table = CString::new("filter").unwrap();
        chain::set(chain, chain::chain_attr::TABLE as u16, table.as_ptr() as *const c_void);
        let name = CString::new("input").unwrap();
        chain::set(chain, chain::chain_attr::NAME as u16, name.as_ptr() as *const c_void);
        chain::nlmsg_build_payload(header, chain);
        chain::free(chain);

        let mut output_type = common::output_type::NFTNL_OUTPUT_JSON;


        let socket = socket::open(socket::NETLINK_NETFILTER);
        assert!(socket != ptr::null_mut());
        assert!(socket::bind(socket, 0, 0) >= 0);
        let port_id = socket::get_portid(socket);

        if socket::sendto(socket, header as *const c_void, (*header).nlmsg_len as usize) < 0 {
            libc::perror(b"sendto".as_ptr() as *const i8);
        }

        loop {
            let received = socket::recvfrom(socket, buf.as_ptr() as *mut c_void, buf.len());
            if received <= 0 {
                break;
            }
            if callback::run(buf.as_ptr() as *const c_void,
                             received as usize,
                             seq,
                             port_id,
                             Some(table_cb),
                             &mut output_type as *mut common::output_type as *mut c_void) <= 0 {
                break;
            }
        }
        libc::perror(b"loop ".as_ptr() as *const i8);
        socket::close(socket);
    }
}

unsafe extern "C" fn table_cb(header: *const nlmsghdr, data: *mut c_void) -> i32 {
    let mut buf = [0; 4096];
    let chain = chain::alloc();
    assert!(chain != ptr::null_mut());
    if chain::nlmsg_parse(header, chain) < 0 {
        libc::perror(b"parse ".as_ptr() as *const i8);
    }
    let mut output_type = common::output_type::NFTNL_OUTPUT_JSON;
    chain::snprintf(buf.as_mut_ptr(), buf.len(), chain, output_type as u32, 0);
    let string = CStr::from_ptr(buf.as_ptr());
    println!("{:?}", string);
    let pote = CStr::from_ptr(chain::get_str(chain, 5));
    println!("{:?}", pote);
    0
}
