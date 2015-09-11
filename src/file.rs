//!File related utilities.

use std::path::{Path, Component};

use mime::{Mime, TopLevel, SubLevel};

include!(concat!(env!("OUT_DIR"), "/mime.rs"));

///Returns the MIME type from a given file extension, if known.
///
///The file extension to MIME type mapping is based on [data from the Apache
///server][apache].
///
///```
///use rustful::file::ext_to_mime;
///use rustful::mime::Mime;
///use rustful::mime::TopLevel::Image;
///use rustful::mime::SubLevel::Jpeg;
///
///let mime = ext_to_mime("jpg");
///assert_eq!(mime, Some(Mime(Image, Jpeg, vec![])));
///```
///
///[apache]: http://svn.apache.org/viewvc/httpd/httpd/trunk/docs/conf/mime.types?view=markup
pub fn ext_to_mime(ext: &str) -> Option<Mime> {
    MIME.get(ext).map(|&(ref top, ref sub)| {
        Mime(top.into(), sub.into(), vec![])
    })
}

enum MaybeKnown<T> {
    Known(T),
    Unknown(&'static str)
}

impl<'a> Into<TopLevel> for &'a MaybeKnown<TopLevel> {
    fn into(self) -> TopLevel {
        match *self {
            MaybeKnown::Known(ref t) => t.clone(),
            MaybeKnown::Unknown(t) => TopLevel::Ext(t.into())
        }
    }
}

impl<'a> Into<SubLevel> for &'a MaybeKnown<SubLevel> {
    fn into(self) -> SubLevel {
        match *self {
            MaybeKnown::Known(ref s) => s.clone(),
            MaybeKnown::Unknown(s) => SubLevel::Ext(s.into())
        }
    }
}

///Check if a path tries to escape its parent directory.
///
///Forbidden path components:
///
/// * Root directory
/// * Prefixes (e.g. `C:` on Windows)
/// * Parent directory
///
///Allowed path components:
///
/// * "Normal" components (e.g. `res/scripts`)
/// * Current directory
///
///The first forbidden component is returned if the path is invalid.
///
///```
///use std::path::Component;
///use rustful::file::check_path;
///
///let bad_path = "..";
///
///assert_eq!(check_path(bad_path), Err(Component::ParentDir));
///```
pub fn check_path<P: ?Sized + AsRef<Path>>(path: &P) -> Result<(), Component> {
    for component in path.as_ref().components() {
        match component {
            c @ Component::RootDir |
            c @ Component::Prefix(_) |
            c @ Component::ParentDir => return Err(c),
            Component::Normal(_) | Component::CurDir => {}
        }
    }

    Ok(())
}
