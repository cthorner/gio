// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gio_sys;
use glib;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::GString;
use glib::StaticType;
use glib::ToValue;
use glib::Value;
use glib_sys;
use gobject_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use std::ptr;
use Converter;

glib_wrapper! {
    pub struct CharsetConverter(Object<gio_sys::GCharsetConverter, gio_sys::GCharsetConverterClass, CharsetConverterClass>) @implements Converter;

    match fn {
        get_type => || gio_sys::g_charset_converter_get_type(),
    }
}

impl CharsetConverter {
    pub fn new(to_charset: &str, from_charset: &str) -> Result<CharsetConverter, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = gio_sys::g_charset_converter_new(
                to_charset.to_glib_none().0,
                from_charset.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}

#[derive(Clone, Default)]
pub struct CharsetConverterBuilder {
    from_charset: Option<String>,
    to_charset: Option<String>,
    use_fallback: Option<bool>,
}

impl CharsetConverterBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> CharsetConverter {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref from_charset) = self.from_charset {
            properties.push(("from-charset", from_charset));
        }
        if let Some(ref to_charset) = self.to_charset {
            properties.push(("to-charset", to_charset));
        }
        if let Some(ref use_fallback) = self.use_fallback {
            properties.push(("use-fallback", use_fallback));
        }
        glib::Object::new(CharsetConverter::static_type(), &properties)
            .expect("object new")
            .downcast()
            .expect("downcast")
    }

    pub fn from_charset(mut self, from_charset: &str) -> Self {
        self.from_charset = Some(from_charset.to_string());
        self
    }

    pub fn to_charset(mut self, to_charset: &str) -> Self {
        self.to_charset = Some(to_charset.to_string());
        self
    }

    pub fn use_fallback(mut self, use_fallback: bool) -> Self {
        self.use_fallback = Some(use_fallback);
        self
    }
}

pub const NONE_CHARSET_CONVERTER: Option<&CharsetConverter> = None;

pub trait CharsetConverterExt: 'static {
    fn get_num_fallbacks(&self) -> u32;

    fn get_use_fallback(&self) -> bool;

    fn set_use_fallback(&self, use_fallback: bool);

    fn get_property_from_charset(&self) -> Option<GString>;

    fn get_property_to_charset(&self) -> Option<GString>;

    fn connect_property_use_fallback_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;
}

impl<O: IsA<CharsetConverter>> CharsetConverterExt for O {
    fn get_num_fallbacks(&self) -> u32 {
        unsafe { gio_sys::g_charset_converter_get_num_fallbacks(self.as_ref().to_glib_none().0) }
    }

    fn get_use_fallback(&self) -> bool {
        unsafe {
            from_glib(gio_sys::g_charset_converter_get_use_fallback(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_use_fallback(&self, use_fallback: bool) {
        unsafe {
            gio_sys::g_charset_converter_set_use_fallback(
                self.as_ref().to_glib_none().0,
                use_fallback.to_glib(),
            );
        }
    }

    fn get_property_from_charset(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"from-charset\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `from-charset` getter")
        }
    }

    fn get_property_to_charset(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"to-charset\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `to-charset` getter")
        }
    }

    fn connect_property_use_fallback_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_use_fallback_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gio_sys::GCharsetConverter,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<CharsetConverter>,
        {
            let f: &F = &*(f as *const F);
            f(&CharsetConverter::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::use-fallback\0".as_ptr() as *const _,
                Some(transmute(
                    notify_use_fallback_trampoline::<Self, F> as usize,
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for CharsetConverter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CharsetConverter")
    }
}
