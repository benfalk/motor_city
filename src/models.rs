use std::ffi::CString;
use std::os::raw::c_char;

#[derive(Queryable, Debug)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Debug)]
#[repr(C)]
pub struct RubyPost {
    pub id: i32,
    pub title: *mut c_char,
    pub body: *mut c_char,
    pub published: bool,
}

impl From<Post> for RubyPost {
    fn from(post: Post) -> Self {
        let title = unsafe { CString::from_vec_unchecked(post.title.into_bytes()) };
        let body = unsafe { CString::from_vec_unchecked(post.body.into_bytes()) };

        Self {
            id: post.id,
            title: title.into_raw(),
            body: body.into_raw(),
            published: post.published,
        }
    }
}
