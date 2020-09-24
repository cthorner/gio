// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gio_sys;
#[cfg(any(feature = "v2_44", feature = "dox"))]
use glib;
#[cfg(any(feature = "v2_44", feature = "dox"))]
use glib::object::Cast;
#[cfg(any(feature = "v2_44", feature = "dox"))]
use glib::object::IsA;
use glib::translate::*;
#[cfg(any(feature = "v2_44", feature = "dox"))]
use glib::StaticType;
#[cfg(any(feature = "v2_44", feature = "dox"))]
use glib::ToValue;
use std::fmt;
use ListModel;

glib_wrapper! {
    pub struct ListStore(Object<gio_sys::GListStore, gio_sys::GListStoreClass, ListStoreClass>) @implements ListModel;

    match fn {
        get_type => || gio_sys::g_list_store_get_type(),
    }
}

impl ListStore {
    #[cfg(any(feature = "v2_44", feature = "dox"))]
    pub fn new(item_type: glib::types::Type) -> ListStore {
        unsafe { from_glib_full(gio_sys::g_list_store_new(item_type.to_glib())) }
    }
}

#[derive(Clone, Default)]
pub struct ListStoreBuilder {
    #[cfg(any(feature = "v2_44", feature = "dox"))]
    item_type: Option<glib::types::Type>,
}

impl ListStoreBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> ListStore {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        #[cfg(any(feature = "v2_44", feature = "dox"))]
        {
            if let Some(ref item_type) = self.item_type {
                properties.push(("item-type", item_type));
            }
        }
        glib::Object::new(ListStore::static_type(), &properties)
            .expect("object new")
            .downcast()
            .expect("downcast")
    }

    #[cfg(any(feature = "v2_44", feature = "dox"))]
    pub fn item_type(mut self, item_type: glib::types::Type) -> Self {
        self.item_type = Some(item_type);
        self
    }
}

pub const NONE_LIST_STORE: Option<&ListStore> = None;

pub trait ListStoreExt: 'static {
    #[cfg(any(feature = "v2_44", feature = "dox"))]
    fn append<P: IsA<glib::Object>>(&self, item: &P);

    #[cfg(any(feature = "v2_44", feature = "dox"))]
    fn insert<P: IsA<glib::Object>>(&self, position: u32, item: &P);

    #[cfg(any(feature = "v2_44", feature = "dox"))]
    fn remove(&self, position: u32);

    #[cfg(any(feature = "v2_44", feature = "dox"))]
    fn remove_all(&self);

    #[cfg(any(feature = "v2_44", feature = "dox"))]
    fn splice(&self, position: u32, n_removals: u32, additions: &[glib::Object]);
}

impl<O: IsA<ListStore>> ListStoreExt for O {
    #[cfg(any(feature = "v2_44", feature = "dox"))]
    fn append<P: IsA<glib::Object>>(&self, item: &P) {
        unsafe {
            gio_sys::g_list_store_append(
                self.as_ref().to_glib_none().0,
                item.as_ref().to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v2_44", feature = "dox"))]
    fn insert<P: IsA<glib::Object>>(&self, position: u32, item: &P) {
        unsafe {
            gio_sys::g_list_store_insert(
                self.as_ref().to_glib_none().0,
                position,
                item.as_ref().to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v2_44", feature = "dox"))]
    fn remove(&self, position: u32) {
        unsafe {
            gio_sys::g_list_store_remove(self.as_ref().to_glib_none().0, position);
        }
    }

    #[cfg(any(feature = "v2_44", feature = "dox"))]
    fn remove_all(&self) {
        unsafe {
            gio_sys::g_list_store_remove_all(self.as_ref().to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v2_44", feature = "dox"))]
    fn splice(&self, position: u32, n_removals: u32, additions: &[glib::Object]) {
        let n_additions = additions.len() as u32;
        unsafe {
            gio_sys::g_list_store_splice(
                self.as_ref().to_glib_none().0,
                position,
                n_removals,
                additions.to_glib_none().0,
                n_additions,
            );
        }
    }
}

impl fmt::Display for ListStore {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ListStore")
    }
}
