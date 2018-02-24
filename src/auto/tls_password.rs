// This file was generated by gir (https://github.com/gtk-rs/gir @ bd67955)
// from gir-files (https://github.com/gtk-rs/gir-files @ 77d1f70)
// DO NOT EDIT

use TlsPasswordFlags;
use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct TlsPassword(Object<ffi::GTlsPassword, ffi::GTlsPasswordClass>);

    match fn {
        get_type => || ffi::g_tls_password_get_type(),
    }
}

impl TlsPassword {
    pub fn new(flags: TlsPasswordFlags, description: &str) -> TlsPassword {
        unsafe {
            from_glib_full(ffi::g_tls_password_new(flags.to_glib(), description.to_glib_none().0))
        }
    }
}

pub trait TlsPasswordExt {
    fn get_description(&self) -> Option<String>;

    fn get_flags(&self) -> TlsPasswordFlags;

    fn get_warning(&self) -> Option<String>;

    fn set_description(&self, description: &str);

    fn set_flags(&self, flags: TlsPasswordFlags);

    //fn set_value_full<'a, P: Into<Option<&'a /*Ignored*/glib::DestroyNotify>>>(&self, value: u8, length: isize, destroy: P);

    fn set_warning(&self, warning: &str);

    fn connect_property_description_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_flags_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_warning_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<TlsPassword> + IsA<glib::object::Object>> TlsPasswordExt for O {
    fn get_description(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_tls_password_get_description(self.to_glib_none().0))
        }
    }

    fn get_flags(&self) -> TlsPasswordFlags {
        unsafe {
            from_glib(ffi::g_tls_password_get_flags(self.to_glib_none().0))
        }
    }

    fn get_warning(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_tls_password_get_warning(self.to_glib_none().0))
        }
    }

    fn set_description(&self, description: &str) {
        unsafe {
            ffi::g_tls_password_set_description(self.to_glib_none().0, description.to_glib_none().0);
        }
    }

    fn set_flags(&self, flags: TlsPasswordFlags) {
        unsafe {
            ffi::g_tls_password_set_flags(self.to_glib_none().0, flags.to_glib());
        }
    }

    //fn set_value_full<'a, P: Into<Option<&'a /*Ignored*/glib::DestroyNotify>>>(&self, value: u8, length: isize, destroy: P) {
    //    unsafe { TODO: call ffi::g_tls_password_set_value_full() }
    //}

    fn set_warning(&self, warning: &str) {
        unsafe {
            ffi::g_tls_password_set_warning(self.to_glib_none().0, warning.to_glib_none().0);
        }
    }

    fn connect_property_description_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::description",
                transmute(notify_description_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_flags_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::flags",
                transmute(notify_flags_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_warning_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::warning",
                transmute(notify_warning_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_description_trampoline<P>(this: *mut ffi::GTlsPassword, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<TlsPassword> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&TlsPassword::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_flags_trampoline<P>(this: *mut ffi::GTlsPassword, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<TlsPassword> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&TlsPassword::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_warning_trampoline<P>(this: *mut ffi::GTlsPassword, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<TlsPassword> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&TlsPassword::from_glib_borrow(this).downcast_unchecked())
}
