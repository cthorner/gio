// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v2_36", feature = "dox"))]
use File;
#[cfg(any(feature = "v2_34", feature = "dox"))]
use InputStream;
use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct ApplicationCommandLine(Object<ffi::GApplicationCommandLine, ffi::GApplicationCommandLineClass>);

    match fn {
        get_type => || ffi::g_application_command_line_get_type(),
    }
}

pub trait ApplicationCommandLineExt {
    #[cfg(any(feature = "v2_36", feature = "dox"))]
    fn create_file_for_arg<P: AsRef<std::ffi::OsStr>>(&self, arg: P) -> Option<File>;

    fn get_arguments(&self) -> Vec<std::ffi::OsString>;

    fn get_cwd(&self) -> Option<std::path::PathBuf>;

    fn get_environ(&self) -> Vec<std::ffi::OsString>;

    fn get_exit_status(&self) -> i32;

    fn get_is_remote(&self) -> bool;

    //#[cfg(any(feature = "v2_40", feature = "dox"))]
    //fn get_options_dict(&self) -> /*Ignored*/Option<glib::VariantDict>;

    fn get_platform_data(&self) -> Option<glib::Variant>;

    #[cfg(any(feature = "v2_34", feature = "dox"))]
    fn get_stdin(&self) -> Option<InputStream>;

    fn getenv<P: AsRef<std::ffi::OsStr>>(&self, name: P) -> Option<String>;

    //fn print(&self, format: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    //fn printerr(&self, format: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    fn set_exit_status(&self, exit_status: i32);

    fn connect_property_arguments_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_is_remote_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_options_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_platform_data_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<ApplicationCommandLine> + IsA<glib::object::Object>> ApplicationCommandLineExt for O {
    #[cfg(any(feature = "v2_36", feature = "dox"))]
    fn create_file_for_arg<P: AsRef<std::ffi::OsStr>>(&self, arg: P) -> Option<File> {
        unsafe {
            from_glib_full(ffi::g_application_command_line_create_file_for_arg(self.to_glib_none().0, arg.as_ref().to_glib_none().0))
        }
    }

    fn get_arguments(&self) -> Vec<std::ffi::OsString> {
        unsafe {
            let mut argc = mem::uninitialized();
            let ret = FromGlibContainer::from_glib_full_num(ffi::g_application_command_line_get_arguments(self.to_glib_none().0, &mut argc), argc as usize);
            ret
        }
    }

    fn get_cwd(&self) -> Option<std::path::PathBuf> {
        unsafe {
            from_glib_none(ffi::g_application_command_line_get_cwd(self.to_glib_none().0))
        }
    }

    fn get_environ(&self) -> Vec<std::ffi::OsString> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::g_application_command_line_get_environ(self.to_glib_none().0))
        }
    }

    fn get_exit_status(&self) -> i32 {
        unsafe {
            ffi::g_application_command_line_get_exit_status(self.to_glib_none().0)
        }
    }

    fn get_is_remote(&self) -> bool {
        unsafe {
            from_glib(ffi::g_application_command_line_get_is_remote(self.to_glib_none().0))
        }
    }

    //#[cfg(any(feature = "v2_40", feature = "dox"))]
    //fn get_options_dict(&self) -> /*Ignored*/Option<glib::VariantDict> {
    //    unsafe { TODO: call ffi::g_application_command_line_get_options_dict() }
    //}

    fn get_platform_data(&self) -> Option<glib::Variant> {
        unsafe {
            from_glib_full(ffi::g_application_command_line_get_platform_data(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_34", feature = "dox"))]
    fn get_stdin(&self) -> Option<InputStream> {
        unsafe {
            from_glib_full(ffi::g_application_command_line_get_stdin(self.to_glib_none().0))
        }
    }

    fn getenv<P: AsRef<std::ffi::OsStr>>(&self, name: P) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_application_command_line_getenv(self.to_glib_none().0, name.as_ref().to_glib_none().0))
        }
    }

    //fn print(&self, format: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi::g_application_command_line_print() }
    //}

    //fn printerr(&self, format: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi::g_application_command_line_printerr() }
    //}

    fn set_exit_status(&self, exit_status: i32) {
        unsafe {
            ffi::g_application_command_line_set_exit_status(self.to_glib_none().0, exit_status);
        }
    }

    fn connect_property_arguments_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::arguments",
                transmute(notify_arguments_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_is_remote_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::is-remote",
                transmute(notify_is_remote_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_options_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::options",
                transmute(notify_options_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_platform_data_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::platform-data",
                transmute(notify_platform_data_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_arguments_trampoline<P>(this: *mut ffi::GApplicationCommandLine, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ApplicationCommandLine> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ApplicationCommandLine::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_is_remote_trampoline<P>(this: *mut ffi::GApplicationCommandLine, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ApplicationCommandLine> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ApplicationCommandLine::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_options_trampoline<P>(this: *mut ffi::GApplicationCommandLine, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ApplicationCommandLine> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ApplicationCommandLine::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_platform_data_trampoline<P>(this: *mut ffi::GApplicationCommandLine, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ApplicationCommandLine> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ApplicationCommandLine::from_glib_borrow(this).downcast_unchecked())
}
