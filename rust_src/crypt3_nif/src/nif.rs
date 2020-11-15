use rustler::{Binary, Encoder, Env, NifResult, OwnedBinary, Term};

use atoms::{decoding, encoding, error, ok};
use bindings;
use bindings::{EncodedField, EncryptionError};

// ============================================================================
// Resource
// ============================================================================

pub fn on_load(_env: Env, _load_info: Term) -> bool {
    true
}

// ============================================================================
// API
// ============================================================================

#[rustler::nif]
fn encrypt<'a>(env: Env<'a>, password: String, salt: String) -> NifResult<Term<'a>> {
    match bindings::encrypt(password, salt) {
        Ok(encrypted) => Ok((ok(), encrypted).encode(env)),
        Err(e) => match e {
            EncryptionError::Encoding(EncodedField::Key) => {
                Ok((error(), (encoding(), "Could not encode key")).encode(env))
            }
            EncryptionError::Encoding(EncodedField::Salt) => {
                Ok((error(), (encoding(), "Could not encode salt")).encode(env))
            }
            EncryptionError::Decoding(message) => Ok((error(), (decoding(), message)).encode(env)),
        },
    }
}

// ============================================================================
// Helpers
// ============================================================================

/// Represents either a borrowed `Binary` or `OwnedBinary`.
///
/// `LazyBinary` allows for the most efficient conversion from an
/// Erlang term to a byte slice. If the term is an actual Erlang
/// binary, constructing `LazyBinary` is essentially
/// zero-cost. However, if the term is any other Erlang type, it is
/// converted to an `OwnedBinary`, which requires a heap allocation.
enum LazyBinary<'a> {
    Owned(OwnedBinary),
    Borrowed(Binary<'a>),
}

impl<'a> std::ops::Deref for LazyBinary<'a> {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        match self {
            Self::Owned(owned) => owned.as_ref(),
            Self::Borrowed(borrowed) => borrowed.as_ref(),
        }
    }
}

impl<'a> rustler::Decoder<'a> for LazyBinary<'a> {
    fn decode(term: Term<'a>) -> NifResult<Self> {
        if term.is_binary() {
            Ok(Self::Borrowed(Binary::from_term(term)?))
        } else {
            Ok(Self::Owned(term.to_binary()))
        }
    }
}
