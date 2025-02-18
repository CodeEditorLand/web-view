//! Raw FFI bindings to [webview].
//!
//! To use a custom version of webview, define an environment variable
//! `WEBVIEW_DIR` with the path to its source directory.
//!
//! [webview]: https://github.com/zserge/webview

#[cfg(all(target_family = "unix", not(target_os = "macos")))]
mod gtk;

use std::os::raw::*;

pub enum CWebView {} // opaque type, only used in ffi pointers

type ErasedExternalInvokeFn = extern fn(webview:*mut CWebView, arg:*const c_char);
type ErasedDispatchFn = extern fn(webview:*mut CWebView, arg:*mut c_void);

extern {
	pub fn webview_free(this:*mut CWebView);

	pub fn webview_new(
		title:*const c_char,
		url:*const c_char,
		width:c_int,
		height:c_int,
		resizable:c_int,
		debug:c_int,
		frameless:c_int,
		external_invoke_cb:Option<ErasedExternalInvokeFn>,
		userdata:*mut c_void,
	) -> *mut CWebView;

	pub fn webview_loop(this:*mut CWebView, blocking:c_int) -> c_int;

	pub fn webview_exit(this:*mut CWebView);

	pub fn webview_get_user_data(this:*mut CWebView) -> *mut c_void;

	pub fn webview_dispatch(this:*mut CWebView, f:Option<ErasedDispatchFn>, arg:*mut c_void);

	pub fn webview_eval(this:*mut CWebView, js:*const c_char) -> c_int;

	pub fn webview_set_title(this:*mut CWebView, title:*const c_char);

	pub fn webview_set_fullscreen(this:*mut CWebView, fullscreen:c_int);

	pub fn webview_set_color(this:*mut CWebView, red:u8, green:u8, blue:u8, alpha:u8);
}
