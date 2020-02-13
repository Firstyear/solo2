pub use generic_array::GenericArray;

pub use heapless::{
    consts,
    String,
    Vec,
};

pub use heapless_bytes::Bytes;

pub use littlefs2::{
    fs::{Filesystem, FilesystemWith},
    driver::Storage as LfsStorage,
    io::Result as LfsResult,
};

use crate::config::*;

pub use crate::client::FutureResult;

// for counters use the pkcs#11 idea of
// a monotonic incrementing counter that
// "increments on each read" --> save +=1 operation

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct AeadUniqueId {
    unique_id: [u8; 16],
    nonce: [u8; 12],
    tag: [u8; 16],
}

pub type AeadKey = [u8; 32];
pub type AeadNonce = [u8; 12];
pub type AeadTag = [u8; 16];



// Object Hierarchy according to Cryptoki
// - Storage
//   - Domain parameters
//   - Key
//   - Certificate
//   - Data
// - Hardware feature
// - Mechanism
// - Profiles


#[derive(Clone, Eq, PartialEq, Debug)]
pub enum CertificateType {
    // "identity", issued by certificate authority
    // --> authentication
    PublicKey,
    // issued by attribute authority
    // --> authorization
    Attribute,
}

// pub enum CertificateCategory {
//     Authority,
//     Token,
//     Other,
// }

// #[derive(Clone, Default, Eq, PartialEq, Debug)]
// pub struct CertificateAttributes {
//     pub certificate_type CertificateType,
// }


#[derive(Clone, Default, Eq, PartialEq, Debug)]
pub struct DataAttributes {
    // application that manages the object
    // pub application: String<MAX_APPLICATION_NAME_LENGTH>,
    // DER-encoding of *type* of data object
    // pub object_id: Bytes<?>,
    pub value: Bytes<MAX_DATA_LENGTH>,
}

// TODO: In PKCS#11v3, this is a map (AttributeType: ulong -> (*void, len)).
// "An array of CK_ATTRIBUTEs is called a “template” and is used for creating, manipulating and searching for objects."
//
// Maybe we should put these attributes in an enum, and pass an `heapless::IndexSet` of attributes.
// How do we handle defaults?
//
// Lookup seems a bit painful, on the other hand a struct of options is wasteful.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct KeyAttributes {
    // key_type: KeyType,
    // object_id: Bytes,
    // derive: bool, // can other keys be derived
    // local: bool, // generated on token, or copied from such
    // key_gen_mechanism: Mechanism, // only for local, how was key generated
    // allowed_mechanisms: Vec<Mechanism>,

    // never return naked private key
    sensitive: bool,
    // always_sensitive: bool,

    // do not even return wrapped private key
    extractable: bool,
    // never_extractable: bool,

    // do not save to disk
    persistent: bool,
}

impl Default for KeyAttributes {
    fn default() -> Self {
        Self {
            sensitive: true,
            // always_sensitive: true,
            extractable: false,
            // never_extractable: true,
            // cryptoki: token (vs session) object
            // cryptoki: default false
            persistent: false,
        }
    }
}

impl KeyAttributes {
    pub fn new() -> Self {
        Default::default()
    }
}


/// Opaque key handle
///
/// Ideally, this would be authenticated encryption
/// around the information that allows locating the key.
///
/// So e.g. users can't get at keys they don't own
///
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct ObjectHandle{
    pub object_id: UniqueId,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct PublicKeyAttributes {
    // never return naked private key
    sensitive: bool,
    // always_sensitive: bool,

    // do not even return wrapped private key
    extractable: bool,
    // never_extractable: bool,

    // do not save to disk
    persistent: bool,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct PrivateKeyAttributes {
    // never return naked private key
    sensitive: bool,
    // always_sensitive: bool,

    // do not even return wrapped private key
    extractable: bool,
    // never_extractable: bool,

    // do not save to disk
    persistent: bool,
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct StorageAttributes {
    // each object must have a unique ID
    unique_id: UniqueId,

    // description of object
    label: String<MAX_LABEL_LENGTH>,

    // cryptoki: token (vs session) object
    persistent: bool,

    // cryptoki: user must be logged in
    // private: bool,

    modifiable: bool,
    copyable: bool,
    destroyable: bool,

}

impl StorageAttributes {
    pub fn new(unique_id: UniqueId) -> Self {
        Self {
            unique_id,
            label: String::new(),
            persistent: false,

            modifiable: true,
            copyable: true,
            destroyable: true,
        }
    }
}


#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Mechanism {
    Ed25519,
    // P256,
    // X25519,
}

pub type Message = Bytes<MAX_MESSAGE_LENGTH>;

pub type Signature = Bytes<MAX_SIGNATURE_LENGTH>;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct UniqueId(pub(crate) [u8; 16]);

impl UniqueId {
    pub fn hex(&self) -> [u8; 32] {
        let mut hex = [0u8; 32];
        format_hex(&self.0, &mut hex);
        hex
    }
}

// PANICS
const HEX_CHARS: &[u8] = b"0123456789abcdef";
fn format_hex(data: &[u8], mut buffer: &mut [u8]) {
    for byte in data.iter() {
        buffer[0] = HEX_CHARS[(byte >> 4) as usize];
        buffer[1] = HEX_CHARS[(byte & 0xf) as usize];
        buffer = &mut buffer[2..];
    }
}
