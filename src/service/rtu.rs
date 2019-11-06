//use crate::client::Client;
use crate::frame::rtu::{RequestAdu};
use crate::proto::{*, rtu::*};
use crate::slave::*;

use std::future::Future;
use std::io::{Error, ErrorKind};
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_tower::pipeline::Client;
use tower_service::Service;

pub(crate) async fn connect_slave<T>(transport: T, slave: Slave) -> Result<Context<T>, Error>
where
    T: AsyncRead + AsyncWrite + 'static,
{
    let service = make_client_service(transport);

    Ok(Context { 
        service: Box::new(service), 
        slave_id: slave.into() 
    })
}

/// Modbus RTU client
pub(crate) struct Context<T: AsyncRead + AsyncWrite + 'static> {
    service: Client<Framed<T, ClientCodec>, Error, RequestAdu>,
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

    fn call(&self, req: Request) -> Box<dyn Future<Output = Result<Response, Error>>> {
        let disconnect = req == Request::Disconnect;
        let req_adu = self.next_request_adu(req, disconnect);
        let req_hdr = req_adu.hdr;
        Box::new(async {
            let res_adu = self.service.call(req_adu).await?;
                //.map_err(|_e| std::io::Error::new(std::io::ErrorKind::Other, "Res_adu error"))?;

            match res_adu.pdu {
                ResponsePdu(Ok(res)) => verify_response_header(req_hdr, res_adu.hdr).and(Ok(res)),
                ResponsePdu(Err(err)) => Err(Error::new(ErrorKind::Other, err)),
            }
        })
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
