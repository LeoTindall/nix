use std::net::{Ipv4Addr, Ipv6Addr, IpAddr};
use std::mem::transmute;
use libc;
/// Converts a `libc::sockaddr` into an `Option<IpAddr>`.
///
/// It returns `None` if the libc reports a type of address that `std::net`
/// doesn't understand, or if the given `sockaddr_input` was null.
///
/// # Unsafety
///
/// The caller is responsible for guaranteeing that the provided reference
/// refers to valid memory.
pub unsafe fn sockaddr_to_ipaddr(sockaddr_input: *mut libc::sockaddr) -> Option<IpAddr> {
    if let Some(sa) = sockaddr_input.as_ref() {
        // Only IPv4 and IPv6 are supported.
        match sa.sa_family as i32 {
            libc::AF_INET => {
                let data_v4: &libc::sockaddr_in = transmute(sa);

                // Transmuting a u32 into a [u8; 4] because
                // the address is in network byte order.
                let s_addr_v4: [u8; 4] = transmute(data_v4.sin_addr.s_addr);
                Some(IpAddr::V4(Ipv4Addr::from(s_addr_v4)))
            }
            libc::AF_INET6 => {
                let data_v6: &libc::sockaddr_in6;
                // UNSAFETY: Transmuting a sockaddr into a sockaddr_in6.
                // They're the same thing.
                data_v6 = transmute(sa);
                Some(IpAddr::V6(Ipv6Addr::from(data_v6.sin6_addr.s6_addr)))
            }
            _ => None,
        }
    } else {
        None
    }
}