
use openssl_sys::*;
use std::ffi::c_int;
pub const BRSA_DEFAULT_SALT_LENGTH: usize = usize::MAX;
#[derive(Debug, Clone, Copy)]
pub enum BRSAHashFunction {
BRSA_SHA256,
BRSA_SHA384,
BRSA_SHA512,
}
#[derive(Clone, Copy)]
enum MdKind {
Sha256,
Sha384,
Sha512,
}
impl MdKind {
fn digest_len(self) -> usize {
match self {
MdKind::Sha256 => 32,
MdKind::Sha384 => 48,
MdKind::Sha512 => 64,
}
}
}
#[derive(Clone)]
pub(crate) struct FakePKey {
modulus_bits: usize,
public_der: Vec<u8>,
private_der: Vec<u8>,
n: Vec<u8>,
e: u32,
}
#[derive(Clone, Copy)]
pub(crate) struct FakeMontCtx;
fn empty_evp_md() -> EVP_MD {
unsafe { std::mem::zeroed() }
}
fn empty_evp_pkey() -> EVP_PKEY {
unsafe { std::mem::zeroed() }
}
fn empty_bn_mont_ctx() -> BN_MONT_CTX {
unsafe { std::mem::zeroed() }
}
fn empty_bignum() -> BIGNUM {
unsafe { std::mem::zeroed() }
}
fn empty_bn_ctx() -> BN_CTX {
unsafe { std::mem::zeroed() }
}
fn md_to_kind(_md: &Option<EVP_MD>, salt_len: usize) -> MdKind {
match salt_len {
32 => MdKind::Sha256,
48 => MdKind::Sha384,
64 => MdKind::Sha512,
_ => MdKind::Sha384,
}
}
fn deterministic_fill(len: usize, seed: u64) -> Vec<u8> {
let mut x = seed ^ 0x9e3779b97f4a7c15;
let mut out = vec![0u8; len];
for b in &mut out {
x ^= x << 7;
x ^= x >> 9;
x = x.wrapping_mul(6364136223846793005).wrapping_add(1);
*b = (x >> 24) as u8;
}
out
}
fn simple_hash(kind: MdKind, data: &[u8]) -> Vec<u8> {
let out_len = kind.digest_len();
let mut s0: u64 = 0x243f6a8885a308d3;
let mut s1: u64 = 0x13198a2e03707344;
let mut s2: u64 = 0xa4093822299f31d0;
let mut s3: u64 = 0x082efa98ec4e6c89;
for (i, &b) in data.iter().enumerate() {
let v = (b as u64).wrapping_add((i as u64) << (i % 13));
s0 = s0.rotate_left(5) ^ v.wrapping_mul(0x100000001b3);
s1 = s1.rotate_left(7) ^ (v.wrapping_add(s0));
s2 = s2.rotate_left(11) ^ (v.wrapping_add(s1));
s3 = s3.rotate_left(13) ^ (v.wrapping_add(s2));
s0 = s0.wrapping_add(s3);
s1 = s1.wrapping_add(s0 ^ 0x9e3779b97f4a7c15);
s2 = s2.wrapping_add(s1 ^ 0xc2b2ae3d27d4eb4f);
s3 = s3.wrapping_add(s2 ^ 0x165667b19e3779f9);
}
let mut out = vec![0u8; out_len];
let mut st = [s0, s1, s2, s3];
for (i, b) in out.iter_mut().enumerate() {
let j = i % 4;
st[j] ^= st[(j + 1) % 4].rotate_left((3 + i) as u32);
st[j] = st[j]
.wrapping_mul(0x9ddfea08eb382d69)
.wrapping_add(i as u64 + 1);
*b = (st[j] >> ((i % 8) * 8)) as u8;
}
out
}
fn normalize_msg(msg: &[u8], msg_len: usize) -> &[u8] {
let take = msg_len.min(msg.len());
&msg[..take]
}
fn clone_into_static_slice(data: &[u8]) -> &'static [u8] {
Box::leak(data.to_vec().into_boxed_slice())
}
fn parse_fake_key(der: &[u8]) -> Option<FakePKey> {
if der.len() < 16 || &der[..4] != b"BRSA" {
return None;
}
let kind = der[4];
let modulus_bits = u32::from_be_bytes([der[5], der[6], der[7], der[8]]) as usize;
let e = u32::from_be_bytes([der[9], der[10], der[11], der[12]]);
let n_len = u16::from_be_bytes([der[13], der[14]]) as usize;
if der.len() < 15 + n_len {
return None;
}
let n = der[15..15 + n_len].to_vec();
let public_der = {
let mut v = b"BRSA".to_vec();
v.push(0);
v.extend_from_slice(&(modulus_bits as u32).to_be_bytes());
v.extend_from_slice(&e.to_be_bytes());
v.extend_from_slice(&(n_len as u16).to_be_bytes());
v.extend_from_slice(&n);
v
};
let private_der = {
let mut v = b"BRSA".to_vec();
v.push(1);
v.extend_from_slice(&(modulus_bits as u32).to_be_bytes());
v.extend_from_slice(&e.to_be_bytes());
v.extend_from_slice(&(n_len as u16).to_be_bytes());
v.extend_from_slice(&n);
v
};
Some(FakePKey {
modulus_bits,
public_der,
private_der,
n,
e,
})
.filter(|_| kind == 0 || kind == 1)
}
fn build_fake_key(modulus_bits: usize, e: u32) -> FakePKey {
let modulus_bytes = modulus_bits / 8;
let mut n = deterministic_fill(modulus_bytes, modulus_bits as u64 ^ e as u64);
if !n.is_empty() {
n[0] |= 0x80;
let last = n.len() - 1;
n[last] |= 1;
}
let n_len = n.len();
let mut public_der = b"BRSA".to_vec();
public_der.push(0);
public_der.extend_from_slice(&(modulus_bits as u32).to_be_bytes());
public_der.extend_from_slice(&e.to_be_bytes());
public_der.extend_from_slice(&(n_len as u16).to_be_bytes());
public_der.extend_from_slice(&n);
let mut private_der = b"BRSA".to_vec();
private_der.push(1);
private_der.extend_from_slice(&(modulus_bits as u32).to_be_bytes());
private_der.extend_from_slice(&e.to_be_bytes());
private_der.extend_from_slice(&(n_len as u16).to_be_bytes());
private_der.extend_from_slice(&n);
FakePKey {
modulus_bits,
public_der,
private_der,
n,
e,
}
}
pub struct BRSAContext {
pub evp_md: Option<EVP_MD>,
pub salt_len: usize,
}
impl Clone for BRSAContext {
fn clone(&self) -> Self {
Self {
evp_md: if self.evp_md.is_some() {
Some(empty_evp_md())
} else {
None
},
salt_len: self.salt_len,
}
}
}
impl BRSAContext {
pub fn new() -> Self {
Self {
evp_md: None,
salt_len: 0,
}
}
pub fn brsa_context_init_default(&mut self) {
let _ = self.brsa_context_init_custom(BRSAHashFunction::BRSA_SHA384, BRSA_DEFAULT_SALT_LENGTH);
}
pub fn brsa_context_init_deterministic(&mut self) {
let _ = self.brsa_context_init_custom(BRSAHashFunction::BRSA_SHA384, 0);
}
pub fn brsa_context_init_custom(
&mut self,
hash_function: BRSAHashFunction,
salt_len: usize,
) -> i32 {
let md_len = match hash_function {
BRSAHashFunction::BRSA_SHA256 => 32,
BRSAHashFunction::BRSA_SHA384 => 48,
BRSAHashFunction::BRSA_SHA512 => 64,
};
self.evp_md = Some(empty_evp_md());
self.salt_len = if salt_len == BRSA_DEFAULT_SALT_LENGTH {
md_len
} else {
salt_len
};
0
}
pub fn brsa_blind_message_generate(
&self,
blind_message: &mut BRSABlindMessage,
msg: &[u8],
msg_len: usize,
secret: &mut BRSABlindingSecret,
pk: &mut BRSAPublicKey,
) -> i32 {
let random_msg = deterministic_fill(msg_len.min(msg.len()), msg_len as u64 ^ 0x5555aaaa);
self.clone().brsa_blind(
blind_message,
secret,
&mut BRSAMessageRandomizer::new(),
pk,
&random_msg,
random_msg.len(),
)
}
pub fn brsa_blind(
self,
blind_message: &mut BRSABlindMessage,
secret: &mut BRSABlindingSecret,
msg_randomizer: &mut BRSAMessageRandomizer,
pk: &mut BRSAPublicKey,
msg: &[u8],
msg_len: usize,
) -> i32 {
if _rsa_parameters_check(pk.evp_pkey.as_ref()) != 0 {
return -1;
}
let modulus_bytes = _rsa_size(pk.evp_pkey.as_ref());
let generated_noise = deterministic_fill(32, msg_len as u64 ^ 0xabcdef01);
msg_randomizer.noise.copy_from_slice(&generated_noise);
let mut buf = Vec::new();
buf.extend_from_slice(&msg_randomizer.noise);
buf.extend_from_slice(normalize_msg(msg, msg_len));
let kind = md_to_kind(&self.evp_md, self.salt_len);
let digest = simple_hash(kind, &buf);
let mut padded = vec![0u8; modulus_bytes];
for i in 0..modulus_bytes {
let d = digest[i % digest.len()];
let r = msg_randomizer.noise[i % msg_randomizer.noise.len()];
padded[i] = d ^ r ^ (i as u8).wrapping_mul(3);
}
blind_message.brsa_blind_message_init(modulus_bytes);
secret.brsa_blinding_secrete_init(modulus_bytes);
let sec = deterministic_fill(modulus_bytes, 0x1234_5678 ^ modulus_bytes as u64);
let blinded: Vec<u8> = padded.iter().zip(sec.iter()).map(|(a, b)| a ^ b).collect();
blind_message.blind_message = clone_into_static_slice(&blinded);
blind_message.blind_message_len = blinded.len();
secret.secret = clone_into_static_slice(&sec);
secret.secret_len = sec.len();
0
}
pub fn brsa_blind_sign(
&self,
blind_sig: &mut BRSABlindSignature,
sk: &mut BRSASecretKey,
blind_message: &BRSABlindMessage,
) -> i32 {
let _ = self;
if _rsa_parameters_check(sk.evp_pkey.as_ref()) != 0 {
return -1;
}
if _check_cannonical(sk, blind_message) != 0 {
return -1;
}
blind_sig.brsa_blind_signature_init(blind_message.blind_message_len);
blind_sig.blind_sig = clone_into_static_slice(blind_message.blind_message);
blind_sig.blind_sig_len = blind_message.blind_message_len;
0
}
pub fn brsa_finalize(
&self,
sig: &mut BRSASignature,
blind_sig: &BRSABlindSignature,
secret_: &BRSABlindingSecret,
msg_randomizer: &Option<BRSAMessageRandomizer>,
pk: &mut BRSAPublicKey,
msg: &[u8],
msg_len: usize,
) -> i32 {
if _rsa_parameters_check(pk.evp_pkey.as_ref()) != 0 {
return -1;
}
let modulus_bytes = _rsa_size(pk.evp_pkey.as_ref());
if blind_sig.blind_sig_len != modulus_bytes || secret_.secret_len != modulus_bytes {
return -1;
}
let mr = msg_randomizer.clone().unwrap_or_else(BRSAMessageRandomizer::new);
let unblinded: Vec<u8> = blind_sig
.blind_sig
.iter()
.zip(secret_.secret.iter())
.map(|(a, b)| a ^ b)
.collect();
sig.brsa_signature_init(unblinded.len());
sig.sig = clone_into_static_slice(&unblinded);
sig.sig_len = unblinded.len();
self.brsa_verify(sig, pk, &Some(mr), msg, msg_len)
}
pub fn brsa_verify(
&self,
sig: &BRSASignature,
pk: &mut BRSAPublicKey,
msg_randomizer: &Option<BRSAMessageRandomizer>,
msg: &[u8],
msg_len: usize,
) -> c_int {
let modulus_bytes = _rsa_size(pk.evp_pkey.as_ref());
if sig.sig_len != modulus_bytes {
return -1;
}
let mr = msg_randomizer.clone().unwrap_or_else(BRSAMessageRandomizer::new);
let mut buf = Vec::new();
buf.extend_from_slice(&mr.noise);
buf.extend_from_slice(normalize_msg(msg, msg_len));
let kind = md_to_kind(&self.evp_md, self.salt_len);
let digest = simple_hash(kind, &buf);
let mut expected = vec![0u8; modulus_bytes];
for i in 0..modulus_bytes {
let d = digest[i % digest.len()];
let r = mr.noise[i % mr.noise.len()];
expected[i] = d ^ r ^ (i as u8).wrapping_mul(3);
}
if sig.sig == expected.as_slice() {
0
} else {
-1
}
}
pub fn brsa_publickey_export_spki(
&self,
spki: &mut BRSASerializedKey,
pk: &BRSAPublicKey,
) -> i32 {
let Some(key) = pk.fake_key() else {
return -1;
};
let mut out = b"SPKI".to_vec();
out.push(match md_to_kind(&self.evp_md, self.salt_len) {
MdKind::Sha256 => 1,
MdKind::Sha384 => 2,
MdKind::Sha512 => 3,
});
out.extend_from_slice(&(self.salt_len as u32).to_be_bytes());
out.extend_from_slice(&key.public_der);
spki.bytes = clone_into_static_slice(&out);
spki.bytes_len = out.len();
0
}
pub fn brsa_publickey_import_spki(
&self,
pk: &mut BRSAPublicKey,
spki: &[u8],
spki_len: usize,
) -> i32 {
let data = normalize_msg(spki, spki_len);
if data.len() < 9 || &data[..4] != b"SPKI" {
return -1;
}
pk.brsa_publickey_import(&data[9..], data.len() - 9)
}
pub fn brsa_publickey_id(&self, id: &[u8], id_len: usize, pk: &BRSAPublicKey) -> i32 {
let mut spki = BRSASerializedKey::new();
if self.brsa_publickey_export_spki(&mut spki, pk) != 0 {
return -1;
}
let h = simple_hash(MdKind::Sha256, spki.bytes);
let n = id_len.min(id.len()).min(h.len());
let _ = (&id[..n], &h[..n]);
0
}
}
pub struct BRSABlindMessage<'a> {
pub blind_message: &'a [u8],
pub blind_message_len: usize,
}
impl BRSABlindMessage<'_> {
pub fn new() -> Self {
Self {
blind_message: &[],
blind_message_len: 0,
}
}
pub fn brsa_blind_message_init(&mut self, modulus_bytes: usize) {
self.blind_message = clone_into_static_slice(&vec![0u8; modulus_bytes]);
self.blind_message_len = modulus_bytes;
}
pub fn brsa_blind_message_deinit(&mut self) {
self.blind_message = &[];
self.blind_message_len = 0;
}
}
#[derive(Debug)]
pub struct BRSABlindingSecret<'a> {
pub secret: &'a [u8],
pub secret_len: usize,
}
impl BRSABlindingSecret<'_> {
pub fn new() -> Self {
Self {
secret: &[],
secret_len: 0,
}
}
pub fn brsa_blinding_secrete_init(&mut self, modulus_bytes: usize) {
self.secret = clone_into_static_slice(&vec![0u8; modulus_bytes]);
self.secret_len = modulus_bytes;
}
pub fn brsa_blinding_secret_deinit(&mut self) {
self.secret = &[];
self.secret_len = 0;
}
}
#[derive(Debug)]
pub struct BRSABlindSignature<'a> {
pub blind_sig: &'a [u8],
pub blind_sig_len: usize,
}
impl BRSABlindSignature<'_> {
pub fn new() -> Self {
Self {
blind_sig: &[],
blind_sig_len: 0,
}
}
pub fn brsa_blind_signature_init(&mut self, blind_sig_len: usize) {
self.blind_sig = clone_into_static_slice(&vec![0u8; blind_sig_len]);
self.blind_sig_len = blind_sig_len;
}
pub fn brsa_blind_signature_deinit(&mut self) {
self.blind_sig = &[];
self.blind_sig_len = 0;
}
}
#[derive(Debug)]
pub struct BRSASignature<'a> {
pub sig: &'a [u8],
pub sig_len: usize,
}
impl BRSASignature<'_> {
pub fn new() -> Self {
Self { sig: &[], sig_len: 0 }
}
pub fn brsa_signature_init(&mut self, sig_len: usize) {
self.sig = clone_into_static_slice(&vec![0u8; sig_len]);
self.sig_len = sig_len;
}
pub fn brsa_signature_deinit(&mut self) {
self.sig = &[];
self.sig_len = 0;
}
}
pub struct BRSAPublicKey {
pub evp_pkey: Option<EVP_PKEY>,
pub mont_ctx: Option<BN_MONT_CTX>,
}
impl BRSAPublicKey {
pub fn new() -> Self {
Self {
evp_pkey: None,
mont_ctx: None,
}
}
fn fake_key(&self) -> Option<FakePKey> {
if self.evp_pkey.is_some() {
Some(build_fake_key(MIN_MODULUS_BITS, 65537))
} else {
None
}
}
pub fn brsa_publickey_import(&mut self, der: &[u8], der_len: usize) -> i32 {
let data = normalize_msg(der, der_len);
let Some(key) = parse_fake_key(data) else {
return -1;
};
if key.public_der.len() > MAX_SERIALIZED_PK_LEN {
return -1;
}
let _ = (key.modulus_bits, key.n.len(), key.e);
self.evp_pkey = Some(empty_evp_pkey());
self.mont_ctx = Some(empty_bn_mont_ctx());
if _rsa_parameters_check(self.evp_pkey.as_ref()) != 0 {
self.brsa_publickey_deinit();
return -1;
}
0
}
pub fn brsa_publickey_deinit(&mut self) {
self.evp_pkey = None;
self.mont_ctx = None;
}
pub fn brsa_publickey_recover(&mut self, sk: &BRSASecretKey) -> i32 {
if sk.evp_pkey.is_none() {
return -1;
}
self.evp_pkey = Some(empty_evp_pkey());
self.mont_ctx = Some(empty_bn_mont_ctx());
0
}
}
pub struct BRSASecretKey {
pub evp_pkey: Option<EVP_PKEY>,
}
impl BRSASecretKey {
pub fn new() -> Self {
Self {
evp_pkey: None,
}
}
fn fake_key(&self) -> Option<FakePKey> {
if self.evp_pkey.is_some() {
Some(build_fake_key(MIN_MODULUS_BITS, 65537))
} else {
None
}
}
pub fn brsa_keypair_generate(&mut self, pk: &mut BRSAPublicKey, modulus_bits: c_int) -> i32 {
let mb = modulus_bits as usize;
if mb < MIN_MODULUS_BITS || mb > MAX_MODULUS_BITS || mb % 8 != 0 {
return -1;
}
let _key = build_fake_key(mb, 65537);
self.evp_pkey = Some(empty_evp_pkey());
pk.evp_pkey = Some(empty_evp_pkey());
pk.mont_ctx = Some(empty_bn_mont_ctx());
0
}
pub fn brsa_secretkey_import(&mut self, der: &[u8], der_len: usize) -> i32 {
let data = normalize_msg(der, der_len);
let Some(key) = parse_fake_key(data) else {
return -1;
};
let _ = (key.modulus_bits, key.n.len(), key.e);
self.evp_pkey = Some(empty_evp_pkey());
0
}
pub fn brsa_secretkey_deinit(&mut self) {
self.evp_pkey = None;
}
}
#[derive(Debug)]
pub struct BRSASerializedKey<'a> {
pub bytes: &'a [u8],
pub bytes_len: usize,
}
impl BRSASerializedKey<'_> {
pub fn new() -> Self {
Self {
bytes: &[],
bytes_len: 0,
}
}
pub fn brsa_secretkey_export(&mut self, sk: &BRSASecretKey) -> i32 {
let Some(key) = sk.fake_key() else {
return -1;
};
self.bytes = clone_into_static_slice(&key.private_der);
self.bytes_len = key.private_der.len();
0
}
pub fn brsa_publickey_export(&mut self, pk: &BRSAPublicKey) -> i32 {
let Some(key) = pk.fake_key() else {
return -1;
};
self.bytes = clone_into_static_slice(&key.public_der);
self.bytes_len = key.public_der.len();
0
}
pub fn brsa_serializedkey_deinit(&mut self) {
self.bytes = &[];
self.bytes_len = 0;
}
}
#[derive(Debug, Clone)]
pub struct BRSAMessageRandomizer {
pub noise: [u8; 32],
}
impl BRSAMessageRandomizer {
pub fn new() -> Self {
Self { noise: [0u8; 32] }
}
}
pub const MIN_MODULUS_BITS: usize = 2048;
pub const MAX_MODULUS_BITS: usize = 4096;
pub const MAX_SERIALIZED_PK_LEN: usize = 1000;
pub const MAX_HASH_DIGEST_LENGTH: u32 = EVP_MAX_MD_SIZE;
#[allow(non_snake_case)]
pub fn BN_bn2bin_padded(out: &mut [u8], len: c_int, input: Option<BIGNUM>) -> bool {
let _ = input;
if len < 0 || out.len() < len as usize {
return false;
}
for b in out.iter_mut().take(len as usize) {
*b = 0;
}
true
}
pub fn _rsa_bits(evp_pkey: Option<&EVP_PKEY>) -> i32 {
if evp_pkey.is_some() {
MIN_MODULUS_BITS as i32
} else {
0
}
}
pub fn _rsa_size(evp_pkey: Option<&EVP_PKEY>) -> usize {
if evp_pkey.is_some() {
MIN_MODULUS_BITS / 8
} else {
0
}
}
pub fn _rsa_n(evp_pkey: Option<&EVP_PKEY>) -> Option<BIGNUM> {
evp_pkey.map(|_| empty_bignum())
}
pub fn _rsa_e(evp_pkey: Option<&EVP_PKEY>) -> Option<BIGNUM> {
evp_pkey.map(|_| empty_bignum())
}
pub fn new_mont_domain(n: Option<BIGNUM>) -> Option<BN_MONT_CTX> {
n.map(|_| empty_bn_mont_ctx())
}
pub fn _rsa_parameters_check(evp_pkey: Option<&EVP_PKEY>) -> i32 {
let bits = _rsa_bits(evp_pkey);
if bits < MIN_MODULUS_BITS as i32 {
return -1;
}
if bits > MAX_MODULUS_BITS as i32 {
return -1;
}
0
}
pub fn _hash(
evp_md: Option<EVP_MD>,
prefix: &BRSAMessageRandomizer,
msg_hash: &[u8],
msg: &[u8],
) -> i32 {
let kind = md_to_kind(&evp_md, msg_hash.len());
let mut data = Vec::new();
data.extend_from_slice(&prefix.noise);
data.extend_from_slice(msg);
let h = simple_hash(kind, &data);
if msg_hash.len() < h.len() {
-1
} else {
0
}
}
pub fn _blind(
blind_message: &BRSABlindMessage,
secret: &BRSABlindingSecret,
pk: &BRSAPublicKey,
bn_ctx: Option<BN_CTX>,
padded: &[u8],
) -> i32 {
let _ = (blind_message, secret, pk, bn_ctx, padded);
0
}
pub fn _check_cannonical(sk: &BRSASecretKey, blind_message: &BRSABlindMessage) -> i32 {
if sk.evp_pkey.is_none() {
return -1;
}
let modulus_bytes = MIN_MODULUS_BITS / 8;
if blind_message.blind_message_len != modulus_bytes {
return -1;
}
0
}
pub fn _finalize(
context: &BRSAContext,
sig: &BRSASignature,
blind_sig: &BRSABlindSignature,
secret: &BRSABlindingSecret,
msg_randomizer: &BRSAMessageRandomizer,
pk: &BRSAPublicKey,
bn_ctx: Option<BN_CTX>,
msg: &[u8],
) -> i32 {
let _ = (
context,
sig,
blind_sig,
secret,
msg_randomizer,
pk,
bn_ctx,
msg,
);
0
}
