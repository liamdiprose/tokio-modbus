use crate::codec::rtu::{ClientCodec, ServerCodec};
use crate::frame::rtu::{RequestAdu, ResponseAdu};

use std::io::Error;
use tokio_codec::{Decoder, Framed};
use tokio_io::{AsyncRead, AsyncWrite};
//use tokio_proto::pipeline::{ClientProto, ServerProto};
use tower_service::Service;

/// The Tower service for Modbus
pub(crate) struct Proto<S: Service<RequestAdu>> {
    service: S
}

impl<S> Service<RequestAdu> for Proto<S> {
}

impl<T: AsyncRead + AsyncWrite + 'static> ClientProto<T> for Proto {
    type Request = RequestAdu;
    type Response = ResponseAdu;
    type Transport = Framed<T, ClientCodec>;
    type BindTransport = Result<Self::Transport, Error>;

}

impl<T: AsyncRead + AsyncWrite + 'static> ServerProto<T> for Proto {
    type Request = RequestAdu;
    type Response = ResponseAdu;
    type Transport = Framed<T, ServerCodec>;
    type BindTransport = Result<Self::Transport, Error>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(ServerCodec::default().framed(io))
    }
}

#[cfg(test)]
mod tests {
    use super::super::dummy_io::DummyIo;
    use super::Proto;
    use crate::codec::rtu::ClientCodec;

    #[test]
    fn bind_transport() {
        //use tokio_proto::pipeline::ClientProto;
        let proto = Proto;
        let io = DummyIo;
        let parts = proto.bind_transport(io).unwrap().into_parts();
        assert_eq!(parts.codec, ClientCodec::default());
    }
}
