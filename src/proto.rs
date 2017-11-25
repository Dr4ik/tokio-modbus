use frame::{Request, ModbusResult};
use tokio_io::{AsyncRead, AsyncWrite};
use std::io::Error;
use tokio_io::codec::Framed;
use tokio_proto::pipeline::ClientProto;
use codec::tcp::ClientCodec;

pub struct TcpProto;

impl<T: AsyncRead + AsyncWrite + 'static> ClientProto<T> for TcpProto {
    type Request = Request;
    type Response = ModbusResult;
    type Transport = Framed<T, ClientCodec>;
    type BindTransport = Result<Self::Transport, Error>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(ClientCodec::new()))
    }
}