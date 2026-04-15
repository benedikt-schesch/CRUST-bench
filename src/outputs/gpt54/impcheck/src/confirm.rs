use crate::siphash::SipHash;
use crate::trusted_utils::SIG_SIZE_BYTES;

pub fn confirm_result(f_sig: &[u8], constant: u8, out: &mut [u8]) {
let mut key = [0u8; 16];
key.copy_from_slice(&crate::secret::SECRET_KEY[..16]);
let mut sip = SipHash::siphash_init(&key);
sip.siphash_reset();
sip.siphash_update(f_sig, SIG_SIZE_BYTES as u64);
sip.siphash_update(&[constant], 1);
let sig = sip.siphash_digest();
out[..SIG_SIZE_BYTES].copy_from_slice(&sig[..SIG_SIZE_BYTES]);
}
