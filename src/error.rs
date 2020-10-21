use lfs::lfc_res;
use std::{error::Error, fmt};

//
// NOTE: we purposely do not just call this `Error` to prevent
// confusing to users. This name may be ugly, but it's unabiguous that
// it isn't referring to `core::error::Error`.
#[derive(Debug, Clone)]
pub struct LfcError(pub lfc_res);

impl fmt::Display for LfcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let desc = match self.0 {
            lfc_res::lfc_res_ok => "No error",
            lfc_res::lfc_res_err_addr => "Received datagram does not match configured OUI or DID",
            lfc_res::lfc_res_err_fingerprint => {
                "Received datagram's fingerprint does not match locally computed fingerprint"
            }
            lfc_res::lfc_res_err_exception => "Uknown error",
            lfc_res::lfc_res_err_nomem => "Provided buffer is too small for request",
            lfc_res::lfc_res_invalid_type => "Invalid datagram type",
            lfc_res::lfc_res_invalid_flags => "Invalid datagram flags",
        };
        write!(f, "{}", desc)
    }
}
impl Error for LfcError {}
