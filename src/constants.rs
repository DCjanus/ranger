use std::{collections::HashSet, iter::FromIterator};

use once_cell::sync::Lazy;

pub static VIDEO_FORMATS: Lazy<HashSet<&str>> = Lazy::new(|| {
    HashSet::from_iter(vec![
        "webm", "mkv", "flv", "vob", "ogv", "ogg", "drc", "gif", "gifv", "mng", "avi", "mov", "qt",
        "wmv", "yuv", "rm", "rmvb", "asf", "amv", "mp4", "m4p", "m4v", "mpg", "mp2", "mpeg", "mpe",
        "mpv", "mpg", "m2v", "svi", "3gp", "3g2", "mxf", "roq", "nsv", "flv", "f4v", "f4p", "f4a",
        "f4b",
    ])
});
