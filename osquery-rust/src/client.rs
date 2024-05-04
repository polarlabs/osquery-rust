use std::io::Error;
use std::os::unix::net::UnixStream;
use std::time::Duration;

use thrift::protocol::{TBinaryInputProtocol, TBinaryOutputProtocol};

use crate::_osquery as osquery;

pub struct Client {
    client: osquery::ExtensionManagerSyncClient<
        TBinaryInputProtocol<UnixStream>,
        TBinaryOutputProtocol<UnixStream>,
    >,
}

impl Client {
    pub fn new(socket_path: &str, _timeout: Duration) -> Result<Self, Error> {
        // todo: error handling, socket could be unable to connect to
        // todo: use timeout
        let socket_tx = UnixStream::connect(socket_path).unwrap();
        let socket_rx = socket_tx.try_clone().unwrap();

        let in_proto = TBinaryInputProtocol::new(socket_tx, true);
        let out_proto = TBinaryOutputProtocol::new(socket_rx, true);

        Ok(Client {
            client: osquery::ExtensionManagerSyncClient::new(in_proto, out_proto),
        })
    }
}

//
// Extension implements _osquery's Thrift API: trait TExtensionManagerSyncClient
//
impl osquery::TExtensionManagerSyncClient for Client {
    fn extensions(&mut self) -> thrift::Result<osquery::InternalExtensionList> {
        todo!()
    }

    fn options(&mut self) -> thrift::Result<osquery::InternalOptionList> {
        todo!()
    }

    fn register_extension(
        &mut self,
        info: osquery::InternalExtensionInfo,
        registry: osquery::ExtensionRegistry,
    ) -> thrift::Result<osquery::ExtensionStatus> {
        self.client.register_extension(info, registry)
    }

    fn deregister_extension(
        &mut self,
        _uuid: osquery::ExtensionRouteUUID,
    ) -> thrift::Result<osquery::ExtensionStatus> {
        todo!()
    }

    fn query(&mut self, _sql: String) -> thrift::Result<osquery::ExtensionResponse> {
        todo!()
    }

    fn get_query_columns(&mut self, _sql: String) -> thrift::Result<osquery::ExtensionResponse> {
        todo!()
    }
}

//
// Extension implements _osquery's Thrift API: super-trait TExtensionSyncClient
//
impl osquery::TExtensionSyncClient for Client {
    fn ping(&mut self) -> thrift::Result<osquery::ExtensionStatus> {
        Ok(osquery::ExtensionStatus::new(0, "OK".to_string(), Some(0)))
    }

    fn call(
        &mut self,
        _registry: String,
        _item: String,
        _request: osquery::ExtensionPluginRequest,
    ) -> thrift::Result<osquery::ExtensionResponse> {
        todo!()
    }

    fn shutdown(&mut self) -> thrift::Result<()> {
        todo!()
    }
}
