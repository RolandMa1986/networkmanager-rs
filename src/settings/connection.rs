use std::collections::HashMap;

use crate::dbus_api::DBusAccessor;

use crate::gen::OrgFreedesktopNetworkManagerSettingsConnection;

use dbus::arg;

pub struct Connection<'a> {
    dbus_accessor: DBusAccessor<'a>,
}

impl<'a> Connection<'a> {
    pub(crate) fn new(dbus_accessor: DBusAccessor<'a>) -> Self {
        Connection { dbus_accessor }
    }

    pub fn get_settings(
        &self,
    ) -> Result<
        HashMap<String, HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>>,
        dbus::Error,
    > {
        proxy!(self).get_settings()
    }

    pub fn update(
        &self,
        properties: HashMap<&str, HashMap<&str, arg::Variant<Box<dyn arg::RefArg>>>>,
    ) -> Result<(), dbus::Error> {
        proxy!(self).update(properties)
    }
    pub fn update_unsaved(
        &self,
        properties: HashMap<&str, HashMap<&str, arg::Variant<Box<dyn arg::RefArg>>>>,
    ) -> Result<(), dbus::Error> {
        proxy!(self).update_unsaved(properties)
    }
    pub fn delete(&self) -> Result<(), dbus::Error> {
        proxy!(self).delete()
    }
    pub fn get_secrets(
        &self,
        setting_name: &str,
    ) -> Result<
        HashMap<String, HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>>,
        dbus::Error,
    > {
        proxy!(self).get_secrets(setting_name)
    }
    pub fn clear_secrets(&self) -> Result<(), dbus::Error> {
        proxy!(self).clear_secrets()
    }
    pub fn save(&self) -> Result<(), dbus::Error> {
        proxy!(self).save()
    }
    pub fn update2(
        &self,
        settings: HashMap<String, HashMap<String, arg::Variant<Box<dyn arg::RefArg>>>>,
        flags: u32,
        args: HashMap<&str, arg::Variant<Box<dyn arg::RefArg>>>,
    ) -> Result<HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>, dbus::Error> {
        proxy!(self).update2(settings, flags, args)
    }
    pub fn unsaved(&self) -> Result<bool, dbus::Error> {
        proxy!(self).unsaved()
    }
    pub fn flags(&self) -> Result<u32, dbus::Error> {
        proxy!(self).flags()
    }
    pub fn filename(&self) -> Result<String, dbus::Error> {
        proxy!(self).filename()
    }
}
