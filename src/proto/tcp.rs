//use std::io::Error;
//use tokio_proto::pipeline::{ClientProto, ServerProto};

//pub(crate) struct Proto;

use crate::codec::tcp::{ClientCodec, ServerCodec};

use tokio_codec::Framed;
use tokio_io::{AsyncRead, AsyncWrite};

use tokio_tower::{Client, Server};
use tower;

/// Creates a tower service for a client
pub(crate) fn create_client_service<T: AsyncRead + AsyncWrite>(
    transport: T,
) -> impl tower::Service {
    let sink_stream = Framed::new(transport, ClientCodec::default());
    let service = Client::new(sink_stream);

    service
}

/// Creates a tower service for a server
pub(crate) fn create_server_service<T: AsyncRead + AsyncWrite>(
    transport: T,
) -> impl tower::Service {
    let sink_stream = Framed::new(transport, ServerCodec::default());
    let service = Server::new(sink_stream);

    service
}

//#[cfg(test)]
//mod tests {
//    use super::super::dummy_io::DummyIo;
//    use super::Proto;
//    use crate::codec::tcp::ClientCodec;
//
//    #[test]
//    fn bind_transport() {
//        use tokio_proto::pipeline::ClientProto;
//        let proto = Proto;
//        let io = DummyIo;
//        let parts = proto.bind_transport(io).unwrap().into_parts();
//        assert_eq!(parts.codec, ClientCodec::default());
//    }
//}
