use crate::client::Client;
use crate::frame::{rtu::*, *};
use crate::proto::rtu::Proto;
use crate::slave::*;

use std::future::Future;
use std::io::{Error, ErrorKind};
//use tokio_core::reactor::Handle;
use tokio_io::{AsyncRead, AsyncWrite};
//use tokio_proto::pipeline::ClientService;
//use tokio_proto::BindClient;
//use tokio_service::Service;

pub(crate) async fn connect_slave<T>(
    handle: &Handle,
    transport: T,
    slave: Slave,
) -> Result<Context<T>, Error>
where
    T: AsyncRead + AsyncWrite + 'static,
{
    let proto = Proto;
    let service = proto.bind_client(handle, transport);
    let slave_id = slave.into();
    Ok(Context { service, slave_id })
}

/// Modbus RTU client
pub(crate) struct Context<T: AsyncRead + AsyncWrite + 'static> {
    service: ClientService<T, Proto>,
    slave_id: SlaveId,
}

impl<T: AsyncRead + AsyncWrite + 'static> Context<T> {
    fn next_request_adu<R>(&self, req: R, disconnect: bool) -> RequestAdu
    where
        R: Into<RequestPdu>,
    {
        let slave_id = self.slave_id;
        let hdr = Header { slave_id };
        let pdu = req.into();
        RequestAdu {
            hdr,
            pdu,
            disconnect,
        }
    }

    async fn call(&self, req: Request) -> Result<Response, Error> {
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

impl<T: AsyncRead + AsyncWrite + 'static> SlaveContext for Context<T> {
    fn set_slave(&mut self, slave: Slave) {
        self.slave_id = slave.into();
    }
}

impl<T: AsyncRead + AsyncWrite + 'static> Client for Context<T> {
    fn call(&self, req: Request) -> Box<dyn Future<Output = Result<Response, Error>>> {
        Box::new(self.call(req))
    }
}
