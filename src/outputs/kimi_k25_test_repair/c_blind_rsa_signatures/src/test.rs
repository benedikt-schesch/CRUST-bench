use c_blind_rsa_signatures::blind_rsa::*;
fn main() {
let mut context = BRSAContext {
salt_len: 32,
hash_function: BRSAHashFunction::BRSA_SHA256,
evp_md: None,
};
let mut blind_msg = BRSABlindMessage {
blind_message: vec![],
blind_message_len: 0
};
let mut client_secret = BRSABlindingSecret {
secret: vec![],
secret_len: 0
};
let mut blind_sig = BRSABlindSignature {
blind_sig: vec![],
blind_sig_len: 0
};
let mut sig = BRSASignature {
sig: vec![],
sig_len: 0
};
let mut key_id = [0u8; 4];
let mut pk = BRSAPublicKey::new();
assert_eq!(context.brsa_publickey_id(&mut key_id, key_id.len(), &pk), 0);
let mut sk_der = BRSASerializedKey {
bytes: vec![],
bytes_len: 0
};
let mut pk_der = BRSASerializedKey {
bytes: vec![],
bytes_len: 0
};
let mut sk = BRSASecretKey::new();
assert_eq!(sk.brsa_secretkey_import(&sk_der.bytes, sk_der.bytes_len), 0);
assert_eq!(pk.brsa_publickey_import(&pk_der.bytes, pk_der.bytes_len), 0);
let mut context = BRSAContext {
salt_len: 32,
hash_function: BRSAHashFunction::BRSA_SHA256,
evp_md: None,
};
let mut blind_msg = BRSABlindMessage {
blind_message: vec![],
blind_message_len: 0
};
let mut client_secret = BRSABlindingSecret {
secret: vec![],
secret_len: 0
};
let mut blind_sig = BRSABlindSignature {
blind_sig: vec![],
blind_sig_len: 0
};
let mut sig = BRSASignature {
sig: vec![],
sig_len: 0
};
let mut blind_sig2 = BRSABlindSignature {
blind_sig: vec![],
blind_sig_len: 0
};
let mut context = BRSAContext {
salt_len: 32,
hash_function: BRSAHashFunction::BRSA_SHA256,
evp_md: None,
};
let mut blind_msg = BRSABlindMessage {
blind_message: vec![],
blind_message_len: 0
};
let mut client_secret = BRSABlindingSecret {
secret: vec![],
secret_len: 0
};
let mut blind_sig = BRSABlindSignature {
blind_sig: vec![],
blind_sig_len: 0
};
let mut sig = BRSASignature {
sig: vec![],
sig_len: 0
};
let _ = context.brsa_context_init_default();
let msg = b"test message";
let _ = context.brsa_blind_message_generate(&mut blind_msg, msg, msg.len(), &mut client_secret, &mut pk);
}
