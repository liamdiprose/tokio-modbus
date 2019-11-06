use crate::codec::rtu::{ClientCodec, ServerCodec};
use crate::frame::{rtu::*, *};

//use std::io::Error;
use tokio_codec::Framed;
use tokio_io::{AsyncRead, AsyncWrite};

use tokio_tower::pipeline::{Client, Server};
use tower_service;

/// Creates a tower service for a client
pub(crate) fn make_client_service<T: AsyncRead + AsyncWrite + Send + 'static>(
    transport: T,
) -> impl tower_service::Service<RequestAdu> {
    let sink_stream = Framed::new(transport, ClientCodec::default());
    let service = Client::new(sink_stream);

    service
}

/// Creates a tower service for a server
pub(crate) fn make_server_service<T: AsyncRead + AsyncWrite>(
    transport: T,
) -> impl tower_service::Service<RequestAdu> {
    let sink_stream = Framed::new(transport, ServerCodec::default());
    let service = Server::new(sink_stream);

    service
}

//pub(crate) struct Proto;
//
//impl<T: AsyncRead + AsyncWrite + 'static> ClientProto<T> for Proto {
//    type Request = RequestAdu;
//    type Response = ResponseAdu;
//    type Transport = Framed<T, ClientCodec>;
//    type BindTransport = Result<Self::Transport, Error>;
//
//    fn bind_transport(&self, io: T) -> Self::BindTransport {
//        Ok(ClientCodec::default().framed(io))
//    }
//}
//
//impl<T: AsyncRead + AsyncWrite + 'static> ServerProto<T> for Proto {
//    type Request = RequestAdu;
//    type Response = ResponseAdu;
//    type Transport = Framed<T, ServerCodec>;
//    type BindTransport = Result<Self::Transport, Error>;
//
//    fn bind_transport(&self, io: T) -> Self::BindTransport {
//        Ok(ServerCodec::default().framed(io))
//    }
//}
//
//#[cfg(test)]
//mod tests {
//    use super::super::dummy_io::DummyIo;
//    use super::Proto;
//    use crate::codec::rtu::ClientCodec;
//
//    #[test]
//    fn bind_transport() {
//        //use tokio_proto::pipeline::ClientProto;
//        let proto = Proto;
//        let io = DummyIo;
//        let parts = proto.bind_transport(io).unwrap().into_parts();
//        assert_eq!(parts.codec, ClientCodec::default());
//    }
//}
