use spice::{self, MAX_LEN_OUT};

use std::fmt;

#[derive(Debug)]
pub struct SpiceError {
    pub kind: Kind,
    pub long: String,
}

#[derive(Debug, PartialEq)]
pub enum Kind {
    EmptyString,
    NoSuchFile,
    UnknownFrame,
    IdCodeNotFound,
    Unknown,
}

impl From<&str> for Kind {
    fn from(short_err: &str) -> Self {
        match short_err {
            "SPICE(NOSUCHFILE)" => Kind::NoSuchFile,
            "SPICE(EMPTYSTRING)" => Kind::EmptyString,
            "SPICE(UNKNOWNFRAME)" => Kind::UnknownFrame,
            "SPICE(IDCODENOTFOUND)" => Kind::IdCodeNotFound,
            _ => Kind::Unknown,
        }
    }
}

impl std::error::Error for SpiceError {}

impl fmt::Display for SpiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.long)
    }
}

/// Load one or more SPICE kernels into a program.
///
/// # Arguments
///
/// * `name` - A string slice holding the location of the kernel or meta kernel to be loaded.
///
/// # Examples
///
/// ```
/// // Loading an emptystring as a kernel should return an error
/// use spice_results as spice;
/// if let Err(e) = spice::furnsh("") {
///     assert_eq!(e.kind, spice::Kind::EmptyString)
/// } else {
///     panic!("The above should produce an error");
/// }
/// ```
pub fn furnsh(name: &str) -> Result<(), SpiceError> {
    spice::erract("SET", MAX_LEN_OUT as i32, "RETURN");
    spice::furnsh(name);
    if spice::failed() {
        let short = spice::getmsg("SHORT", MAX_LEN_OUT as i32);
        let long = spice::getmsg("LONG", MAX_LEN_OUT as i32);
        let e = SpiceError {
            kind: Kind::from(short.as_str()),
            long,
        };
        spice::reset();
        Err(e)
    } else {
        Ok(())
    }
}
