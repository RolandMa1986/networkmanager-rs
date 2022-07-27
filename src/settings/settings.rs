use crate::dbus_api::DBusAccessor;

use crate::gen::OrgFreedesktopNetworkManagerSettings;

use dbus::arg;

pub struct Settings<'a> {
    dbus_accessor: DBusAccessor<'a>,
}

impl<'a> Settings<'a> {
    pub(crate) fn new(dbus_accessor: DBusAccessor<'a>) -> Self {
        Settings { dbus_accessor }
    }

    pub fn list_connections(&self) -> Result<Vec<dbus::Path<'static>>, dbus::Error> {
        proxy!(self).list_connections()
    }

    pub fn get_connection_by_uuid(&self, uuid: &str) -> Result<dbus::Path<'static>, dbus::Error> {
        proxy!(self).get_connection_by_uuid(uuid)
    }
    pub fn add_connection(
        &self,
        connection: ::std::collections::HashMap<
            &str,
            ::std::collections::HashMap<&str, arg::Variant<Box<dyn arg::RefArg>>>,
        >,
    ) -> Result<dbus::Path<'static>, dbus::Error> {
        proxy!(self).add_connection(connection)
    }
    pub fn add_connection_unsaved(
        &self,
        connection: ::std::collections::HashMap<
            &str,
            ::std::collections::HashMap<&str, arg::Variant<Box<dyn arg::RefArg>>>,
        >,
    ) -> Result<dbus::Path<'static>, dbus::Error> {
        proxy!(self).add_connection_unsaved(connection)
    }
    pub fn add_connection2(
        &self,
        settings: ::std::collections::HashMap<
            &str,
            ::std::collections::HashMap<&str, arg::Variant<Box<dyn arg::RefArg>>>,
        >,
        flags: u32,
        args: ::std::collections::HashMap<&str, arg::Variant<Box<dyn arg::RefArg>>>,
    ) -> Result<
        (
            dbus::Path<'static>,
            ::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>,
        ),
        dbus::Error,
    > {
        proxy!(self).add_connection2(settings, flags, args)
    }
    pub fn load_connections(&self, filenames: Vec<&str>) -> Result<(bool, Vec<String>), dbus::Error> {
        proxy!(self).load_connections(filenames)
    }
    pub fn reload_connections(&self) -> Result<bool, dbus::Error> {
        proxy!(self).reload_connections()
    }
    pub fn save_hostname(&self, hostname: &str) -> Result<(), dbus::Error> {
        proxy!(self).save_hostname(hostname)
    }
    pub fn connections(&self) -> Result<Vec<dbus::Path<'static>>, dbus::Error> {
        proxy!(self).connections()
    }
    pub fn hostname(&self) -> Result<String, dbus::Error> {
        proxy!(self).hostname()
    }
    pub fn can_modify(&self) -> Result<bool, dbus::Error> {
        proxy!(self).can_modify()
    }
}
