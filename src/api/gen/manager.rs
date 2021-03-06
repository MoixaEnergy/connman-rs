// This code was autogenerated with `dbus-codegen-rust -g -i net.connman -m None -c nonblock`, see https://github.com/diwic/dbus-rs
use dbus as dbus;
#[allow(unused_imports)]
use dbus::arg;
use dbus::nonblock;

pub trait OrgFreedesktopDBusIntrospectable {
    fn introspect(&self) -> nonblock::MethodReply<String>;
}

impl<'a, T: nonblock::NonblockReply, C: ::std::ops::Deref<Target=T>> OrgFreedesktopDBusIntrospectable for nonblock::Proxy<'a, C> {

    fn introspect(&self) -> nonblock::MethodReply<String> {
        self.method_call("org.freedesktop.DBus.Introspectable", "Introspect", ())
            .and_then(|r: (String, )| Ok(r.0, ))
    }
}

pub trait Manager {
    fn get_properties(&self) -> nonblock::MethodReply<::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>>;
    fn set_property<I1: arg::Arg + arg::Append>(&self, name: &str, value: I1) -> nonblock::MethodReply<()>;
    fn get_technologies(&self) -> nonblock::MethodReply<Vec<(dbus::Path<'static>, ::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>)>>;
    fn remove_provider(&self, provider: dbus::Path) -> nonblock::MethodReply<()>;
    fn get_services(&self) -> nonblock::MethodReply<Vec<(dbus::Path<'static>, ::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>)>>;
    fn get_peers(&self) -> nonblock::MethodReply<Vec<(dbus::Path<'static>, ::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>)>>;
    fn connect_provider(&self, provider: ::std::collections::HashMap<&str, arg::Variant<Box<dyn arg::RefArg>>>) -> nonblock::MethodReply<dbus::Path<'static>>;
    fn register_agent(&self, path: dbus::Path) -> nonblock::MethodReply<()>;
    fn unregister_agent(&self, path: dbus::Path) -> nonblock::MethodReply<()>;
    fn register_counter(&self, path: dbus::Path, accuracy: u32, period: u32) -> nonblock::MethodReply<()>;
    fn unregister_counter(&self, path: dbus::Path) -> nonblock::MethodReply<()>;
    fn create_session(&self, settings: ::std::collections::HashMap<&str, arg::Variant<Box<dyn arg::RefArg>>>, notifier: dbus::Path) -> nonblock::MethodReply<dbus::Path<'static>>;
    fn destroy_session(&self, session: dbus::Path) -> nonblock::MethodReply<()>;
    fn request_private_network(&self) -> nonblock::MethodReply<(dbus::Path<'static>, ::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>, arg::OwnedFd)>;
    fn release_private_network(&self, path: dbus::Path) -> nonblock::MethodReply<()>;
    fn register_peer_service(&self, specification: ::std::collections::HashMap<&str, arg::Variant<Box<dyn arg::RefArg>>>, master: bool) -> nonblock::MethodReply<()>;
    fn unregister_peer_service(&self, specification: ::std::collections::HashMap<&str, arg::Variant<Box<dyn arg::RefArg>>>) -> nonblock::MethodReply<()>;
}

impl<'a, T: nonblock::NonblockReply, C: ::std::ops::Deref<Target=T>> Manager for nonblock::Proxy<'a, C> {

    fn get_properties(&self) -> nonblock::MethodReply<::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>> {
        self.method_call("net.connman.Manager", "GetProperties", ())
            .and_then(|r: (::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>, )| Ok(r.0, ))
    }

    fn set_property<I1: arg::Arg + arg::Append>(&self, name: &str, value: I1) -> nonblock::MethodReply<()> {
        self.method_call("net.connman.Manager", "SetProperty", (name, arg::Variant(value), ))
    }

    fn get_technologies(&self) -> nonblock::MethodReply<Vec<(dbus::Path<'static>, ::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>)>> {
        self.method_call("net.connman.Manager", "GetTechnologies", ())
            .and_then(|r: (Vec<(dbus::Path<'static>, ::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>)>, )| Ok(r.0, ))
    }

    fn remove_provider(&self, provider: dbus::Path) -> nonblock::MethodReply<()> {
        self.method_call("net.connman.Manager", "RemoveProvider", (provider, ))
    }

    fn get_services(&self) -> nonblock::MethodReply<Vec<(dbus::Path<'static>, ::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>)>> {
        self.method_call("net.connman.Manager", "GetServices", ())
            .and_then(|r: (Vec<(dbus::Path<'static>, ::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>)>, )| Ok(r.0, ))
    }

    fn get_peers(&self) -> nonblock::MethodReply<Vec<(dbus::Path<'static>, ::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>)>> {
        self.method_call("net.connman.Manager", "GetPeers", ())
            .and_then(|r: (Vec<(dbus::Path<'static>, ::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>)>, )| Ok(r.0, ))
    }

    fn connect_provider(&self, provider: ::std::collections::HashMap<&str, arg::Variant<Box<dyn arg::RefArg>>>) -> nonblock::MethodReply<dbus::Path<'static>> {
        self.method_call("net.connman.Manager", "ConnectProvider", (provider, ))
            .and_then(|r: (dbus::Path<'static>, )| Ok(r.0, ))
    }

    fn register_agent(&self, path: dbus::Path) -> nonblock::MethodReply<()> {
        self.method_call("net.connman.Manager", "RegisterAgent", (path, ))
    }

    fn unregister_agent(&self, path: dbus::Path) -> nonblock::MethodReply<()> {
        self.method_call("net.connman.Manager", "UnregisterAgent", (path, ))
    }

    fn register_counter(&self, path: dbus::Path, accuracy: u32, period: u32) -> nonblock::MethodReply<()> {
        self.method_call("net.connman.Manager", "RegisterCounter", (path, accuracy, period, ))
    }

    fn unregister_counter(&self, path: dbus::Path) -> nonblock::MethodReply<()> {
        self.method_call("net.connman.Manager", "UnregisterCounter", (path, ))
    }

    fn create_session(&self, settings: ::std::collections::HashMap<&str, arg::Variant<Box<dyn arg::RefArg>>>, notifier: dbus::Path) -> nonblock::MethodReply<dbus::Path<'static>> {
        self.method_call("net.connman.Manager", "CreateSession", (settings, notifier, ))
            .and_then(|r: (dbus::Path<'static>, )| Ok(r.0, ))
    }

    fn destroy_session(&self, session: dbus::Path) -> nonblock::MethodReply<()> {
        self.method_call("net.connman.Manager", "DestroySession", (session, ))
    }

    fn request_private_network(&self) -> nonblock::MethodReply<(dbus::Path<'static>, ::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>, arg::OwnedFd)> {
        self.method_call("net.connman.Manager", "RequestPrivateNetwork", ())
    }

    fn release_private_network(&self, path: dbus::Path) -> nonblock::MethodReply<()> {
        self.method_call("net.connman.Manager", "ReleasePrivateNetwork", (path, ))
    }

    fn register_peer_service(&self, specification: ::std::collections::HashMap<&str, arg::Variant<Box<dyn arg::RefArg>>>, master: bool) -> nonblock::MethodReply<()> {
        self.method_call("net.connman.Manager", "RegisterPeerService", (specification, master, ))
    }

    fn unregister_peer_service(&self, specification: ::std::collections::HashMap<&str, arg::Variant<Box<dyn arg::RefArg>>>) -> nonblock::MethodReply<()> {
        self.method_call("net.connman.Manager", "UnregisterPeerService", (specification, ))
    }
}

#[derive(Debug)]
pub struct ManagerPropertyChanged {
    pub name: String,
    pub value: arg::Variant<Box<dyn arg::RefArg + 'static>>,
}

impl arg::AppendAll for ManagerPropertyChanged {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.name, i);
        arg::RefArg::append(&self.value, i);
    }
}

impl arg::ReadAll for ManagerPropertyChanged {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(ManagerPropertyChanged {
            name: i.read()?,
            value: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for ManagerPropertyChanged {
    const NAME: &'static str = "PropertyChanged";
    const INTERFACE: &'static str = "net.connman.Manager";
}

#[derive(Debug)]
pub struct ManagerTechnologyAdded {
    pub path: dbus::Path<'static>,
    pub properties: ::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>,
}

impl arg::AppendAll for ManagerTechnologyAdded {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.path, i);
        arg::RefArg::append(&self.properties, i);
    }
}

impl arg::ReadAll for ManagerTechnologyAdded {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(ManagerTechnologyAdded {
            path: i.read()?,
            properties: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for ManagerTechnologyAdded {
    const NAME: &'static str = "TechnologyAdded";
    const INTERFACE: &'static str = "net.connman.Manager";
}

#[derive(Debug)]
pub struct ManagerTechnologyRemoved {
    pub path: dbus::Path<'static>,
}

impl arg::AppendAll for ManagerTechnologyRemoved {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.path, i);
    }
}

impl arg::ReadAll for ManagerTechnologyRemoved {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(ManagerTechnologyRemoved {
            path: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for ManagerTechnologyRemoved {
    const NAME: &'static str = "TechnologyRemoved";
    const INTERFACE: &'static str = "net.connman.Manager";
}

#[derive(Debug)]
pub struct ManagerServicesChanged {
    pub changed: Vec<(dbus::Path<'static>, ::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>)>,
    pub removed: Vec<dbus::Path<'static>>,
}

impl arg::AppendAll for ManagerServicesChanged {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.changed, i);
        arg::RefArg::append(&self.removed, i);
    }
}

impl arg::ReadAll for ManagerServicesChanged {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(ManagerServicesChanged {
            changed: i.read()?,
            removed: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for ManagerServicesChanged {
    const NAME: &'static str = "ServicesChanged";
    const INTERFACE: &'static str = "net.connman.Manager";
}

#[derive(Debug)]
pub struct ManagerPeersChanged {
    pub changed: Vec<(dbus::Path<'static>, ::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>)>,
    pub removed: Vec<dbus::Path<'static>>,
}

impl arg::AppendAll for ManagerPeersChanged {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.changed, i);
        arg::RefArg::append(&self.removed, i);
    }
}

impl arg::ReadAll for ManagerPeersChanged {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(ManagerPeersChanged {
            changed: i.read()?,
            removed: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for ManagerPeersChanged {
    const NAME: &'static str = "PeersChanged";
    const INTERFACE: &'static str = "net.connman.Manager";
}

pub trait Clock {
    fn get_properties(&self) -> nonblock::MethodReply<::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>>;
    fn set_property<I1: arg::Arg + arg::Append>(&self, name: &str, value: I1) -> nonblock::MethodReply<()>;
}

impl<'a, T: nonblock::NonblockReply, C: ::std::ops::Deref<Target=T>> Clock for nonblock::Proxy<'a, C> {

    fn get_properties(&self) -> nonblock::MethodReply<::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>> {
        self.method_call("net.connman.Clock", "GetProperties", ())
            .and_then(|r: (::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>, )| Ok(r.0, ))
    }

    fn set_property<I1: arg::Arg + arg::Append>(&self, name: &str, value: I1) -> nonblock::MethodReply<()> {
        self.method_call("net.connman.Clock", "SetProperty", (name, arg::Variant(value), ))
    }
}

#[derive(Debug)]
pub struct ClockPropertyChanged {
    pub name: String,
    pub value: arg::Variant<Box<dyn arg::RefArg + 'static>>,
}

impl arg::AppendAll for ClockPropertyChanged {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.name, i);
        arg::RefArg::append(&self.value, i);
    }
}

impl arg::ReadAll for ClockPropertyChanged {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(ClockPropertyChanged {
            name: i.read()?,
            value: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for ClockPropertyChanged {
    const NAME: &'static str = "PropertyChanged";
    const INTERFACE: &'static str = "net.connman.Clock";
}
