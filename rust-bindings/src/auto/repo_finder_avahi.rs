// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files
// DO NOT EDIT

use crate::{ffi,RepoFinder};
#[cfg(feature = "v2018_6")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2018_6")))]
use glib::{translate::*};

glib::wrapper! {
    #[doc(alias = "OstreeRepoFinderAvahi")]
    pub struct RepoFinderAvahi(Object<ffi::OstreeRepoFinderAvahi, ffi::OstreeRepoFinderAvahiClass>) @implements RepoFinder;

    match fn {
        type_ => || ffi::ostree_repo_finder_avahi_get_type(),
    }
}

impl RepoFinderAvahi {
    #[cfg(feature = "v2018_6")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2018_6")))]
    #[doc(alias = "ostree_repo_finder_avahi_new")]
    pub fn new(context: Option<&glib::MainContext>) -> RepoFinderAvahi {
        unsafe {
            from_glib_full(ffi::ostree_repo_finder_avahi_new(context.to_glib_none().0))
        }
    }

    #[cfg(feature = "v2018_6")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2018_6")))]
    #[doc(alias = "ostree_repo_finder_avahi_stop")]
    pub fn stop(&self) {
        unsafe {
            ffi::ostree_repo_finder_avahi_stop(self.to_glib_none().0);
        }
    }
}
