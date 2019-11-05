//! Connecting a Modbus RTU context

use super::*;

use crate::service;

use std::io::Error;
//use tokio_core::reactor::Handle;
use tokio_io::{AsyncRead, AsyncWrite};

/// Connect to no particular Modbus slave device for sending
/// broadcast messages.
pub async fn connect<T>(transport: T) -> Result<Context, Error>
where
    T: AsyncRead + AsyncWrite + 'static,
{
    connect_slave(transport, Slave::broadcast()).await
}

/// Connect to any kind of Modbus slave device.
pub async fn connect_slave<T>(transport: T, slave: Slave) -> Result<Context, Error>
where
    T: AsyncRead + AsyncWrite + 'static,
{
    let client = service::rtu::connect_slave(transport, slave).await?;
    Ok(Context {
        client: Box::new(client),
    })
}
