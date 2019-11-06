//use crate::client::Client;
use crate::frame::{tcp::*, *};
use crate::proto::{make_client_service, make_server_service};
use crate::slave::*;
use tokio_tower::Client;

use std::cell::Cell;
use std::future::Future;
use std::io::{Error, ErrorKind};
use std::net::SocketAddr;
use tokio::net::TcpListener;
//use tokio_core::net::TcpStream;
//use tokio_core::reactor::Handle;
//use tokio_proto::pipeline::ClientService;
//use tokio_proto::TcpClient;
//use tokio_service::Service;

pub(crate) async fn connect_slave(socket_addr: SocketAddr, slave: Slave) -> Result<Context, Error> {
    let unit_id = slave.into();
    let transport = TcpListener::bind(&socket_addr);

    let service = make_client_service(transport);

    Ok(Context::new(service, unit_id))
}

const INITIAL_TRANSACTION_ID: TransactionId = 0;

/// Modbus TCP client
pub(crate) struct Context {
    service: Client<TcpListener, io::Error, RequestAdu>,
    unit_id: UnitId,
    transaction_id: Cell<TransactionId>,
}

impl Context {
    fn new(service: Client<TcpListener, io::Error, RequestAdu>, unit_id: UnitId) -> Self {
        Self {
            service,
            unit_id,
            transaction_id: Cell::new(INITIAL_TRANSACTION_ID),
        }
    }

    fn next_transaction_id(&self) -> TransactionId {
        let transaction_id = self.transaction_id.get();
        self.transaction_id.set(transaction_id.wrapping_add(1));
        transaction_id
    }

    fn next_request_hdr(&self, unit_id: UnitId) -> Header {
        let transaction_id = self.next_transaction_id();
        Header {
            transaction_id,
            unit_id,
        }
    }

    fn next_request_adu<R>(&self, req: R, disconnect: bool) -> RequestAdu
    where
        R: Into<RequestPdu>,
    {
        RequestAdu {
            hdr: self.next_request_hdr(self.unit_id),
            pdu: req.into(),
            disconnect,
        }
    }

    pub async fn call(&self, req: Request) -> Result<Response, Error> {
        let disconnect = req == Request::Disconnect;
        let req_adu = self.next_request_adu(req, disconnect);
        let req_hdr = req_adu.hdr;
        let res_adu = self.service.call(req_adu).await;

        match res_adu.pdu {
            ResponsePdu(Ok(res)) => verify_response_header(req_hdr, res_adu.hdr).and(Ok(res)),
            ResponsePdu(Err(err)) => Err(Error::new(ErrorKind::Other, err)),
        }
    }
}

fn verify_response_header(req_hdr: Header, rsp_hdr: Header) -> Result<(), Error> {
    if req_hdr != rsp_hdr {
        return Err(Error::new(
            ErrorKind::InvalidData,
            format!(
                "Invalid response header: expected/request = {:?}, actual/response = {:?}",
                req_hdr, rsp_hdr
            ),
        ));
    }
    Ok(())
}

impl SlaveContext for Context {
    fn set_slave(&mut self, slave: Slave) {
        self.unit_id = slave.into();
    }
}

impl Client for Context {
    fn call(&self, req: Request) -> Box<dyn Future<Output = Result<Response, Error>>> {
        Box::new(self.call(req))
    }
}
