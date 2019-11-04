//! Connecting a Modbus TCP context

use super::*;

use crate::service;

//use futures::Future;
use std::io::Error;
use std::net::SocketAddr;
//use tokio_core::reactor::Handle;

/// Establish a direct connection to a Modbus TCP coupler.
pub async fn connect(handle: &Handle, socket_addr: SocketAddr) -> Result<Context, Error> {
    let context = connect_slave(handle, socket_addr, Slave::tcp_device()).await?;
    Ok(context)
}

/// Connect to a physical, broadcast, or custom Modbus device,
/// probably through a Modbus TCP gateway that is forwarding
/// messages to/from the corresponding slave device.
pub async fn connect_slave(
    handle: &Handle,
    socket_addr: SocketAddr,
    slave: Slave,
) -> Result<Context, Error> {
    let context = service::tcp::connect_slave(handle, socket_addr, slave).await?;
    Ok(Context {
        client: Box::new(context),
    })
}
