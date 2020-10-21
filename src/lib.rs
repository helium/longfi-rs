//! LongFi
//!
//! This crate exposes one single subset of the original LongFi
//! protocol: Monolithic Datagram encoding/decoding. Because of that,
//! the primary type exported is simple called `Datagram`.

mod error;
pub use error::LfcError;

/// LongFi datagram.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Datagram {
    /// Additional flags.
    flags: Flags,
    /// Organization ID.
    oui: u32,
    /// Device ID.
    did: u32,
    /// Fingerprint.
    ///
    /// `fp` is meant to be derived from the rest of the fields and a
    /// session key, and not explicitly set.
    fp: u32,
    /// Sequence number.
    seq: u32,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Flags {
    /// This packet is destined for a Device if this bit is set.
    downlink: bool,
    /// The receiver of this packet should acknowledge receipt.
    should_ack: bool,
    /// On uplink this bit indicates the device is ready to
    /// receive, on downlink it indicates further information
    /// follows.
    cts_rts: bool,
    /// This indicates to the receiver that the packet is deemed
    /// urgent by the sender and the receiver can choose to act
    /// accordingly.
    priority: bool,
    /// The packet, beyond the Tag field, is encoded with a Low
    /// Density Parity Code. The specific code used depends on the
    /// maximum datagram size for the current region and spreading
    /// factor.
    ldpc: bool,
}

impl Datagram {
    /// Encodes this datagram and payload into `dst` as a LongFi frame
    /// suitable for OTA transmission.
    ///
    /// # Errors
    ///
    /// Returns encoded size on success or an error.
    pub fn encode(&self, payload: &[u8], dst: &mut [u8]) -> Result<usize, LfcError> {
        let mono = lfs::lfc_dg_monolithic {
            oui: self.oui,
            did: self.did,
            fp: self.fp,
            seq: self.seq,
            pay_len: payload.len() as lfs::size_t,
            pay: {
                let mut buf = [0_u8; 128];
                buf[..payload.len()].copy_from_slice(payload);
                buf
            },
            flags: lfs::lfc_dg_monolithic_lfc_dg_monolithic_flags {
                downlink: self.flags.downlink,
                should_ack: self.flags.should_ack,
                cts_rts: self.flags.cts_rts,
                priority: self.flags.priority,
                ldpc: self.flags.ldpc,
            },
        };

        let (res, serialzed_sz) = unsafe {
            let mut cursor = lfs::cursor_new(dst.as_ptr() as *mut _, dst.len() as lfs::size_t);
            let res = lfs::lfc_dg_monolithic__ser(&mono, &mut cursor);
            (res, cursor.pos as usize)
        };

        if res != lfs::lfc_res::lfc_res_ok {
            Err(LfcError(res))
        } else {
            Ok(serialzed_sz)
        }
    }

    /// Decodes a datagram from the provided buffer.
    ///
    /// LongFi does not embed encoded payload sizes into the frame
    /// itself thus this method will consume the entirety of `src`. It
    /// is up to the caller that `src` is sliced to exactly the
    /// received LongFi frame and does not contain any extra trailing
    /// data.
    ///
    /// # Errors
    ///
    /// Returns a tuple of decoded payload len and Datagram on
    /// success, else returns the decoded datagram.
    pub fn decode(src: &[u8], payload: &mut [u8]) -> Result<(usize, Self), LfcError> {
        use lfs::lfc_res::*;
        use std::mem::MaybeUninit;

        let mut dg = MaybeUninit::<lfs::lfc_dg_des>::uninit();
        let res = unsafe {
            let mut cursor = lfs::cursor_new(src.as_ptr() as *mut _, src.len() as lfs::size_t);
            lfs::lfc_dg__des(dg.as_mut_ptr(), &mut cursor)
        };
        if res != lfc_res_ok {
            return Err(LfcError(res));
        }
        let dg = unsafe { dg.assume_init() };

        if dg.type_ != lfs::lfc_dg_type_lfc_dg_type_monolithic {
            return Err(LfcError(lfc_res_err_exception));
        }

        let mono = unsafe { dg.__bindgen_anon_1.monolithic.as_ref() };
        let dec_pay_len = mono.pay_len as usize;

        if dec_pay_len > payload.len() {
            return Err(LfcError(lfc_res_err_nomem));
        }

        payload[..dec_pay_len].copy_from_slice(unsafe {
            std::slice::from_raw_parts(&mono.pay as *const u8, dec_pay_len)
        });

        Ok((
            dec_pay_len,
            Self {
                oui: mono.oui,
                did: mono.did,
                fp: mono.fp,
                seq: mono.seq,
                flags: Flags {
                    downlink: mono.flags.downlink,
                    should_ack: mono.flags.should_ack,
                    cts_rts: mono.flags.cts_rts,
                    priority: mono.flags.priority,
                    ldpc: mono.flags.ldpc,
                },
            },
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_datagram_roundtrip() {
        let dg = Datagram {
            flags: Flags {
                downlink: true,
                should_ack: false,
                cts_rts: true,
                priority: false,
                ldpc: true,
            },
            oui: 1,
            did: 2,
            fp: 3,
            seq: 4,
        };
        let pay: Vec<u8> = (0..64).collect();
        let mut encoded: Vec<u8> = (0_u8..128).rev().collect();
        let encoded_len = dg.encode(&pay, &mut encoded).unwrap();
        let mut dec_pay = [0xFE; 65];
        let (dec_pay_len, dec_dg) =
            Datagram::decode(&encoded[..encoded_len], &mut dec_pay).unwrap();

        assert!(encoded_len > pay.len());
        assert_eq!(dg, dec_dg);
        assert_eq!(pay[..], dec_pay[..dec_pay_len]);
    }
}
