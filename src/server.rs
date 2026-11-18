use std::{net::Ipv4Addr, ops::Deref};

use async_trait::async_trait;
use color_eyre::{
    Help,
    eyre::{Report, Result},
};
use hickory_proto::op::HeaderCounts;
use hickory_server::{
    proto::{
        op::{Header, Metadata, ResponseCode},
        rr::{RData, Record, rdata::A},
    },
    server::{Request, RequestHandler, ResponseHandler, ResponseInfo, Server},
    zone_handler::{MessageRequest, MessageResponseBuilder},
};
use tokio::net::UdpSocket;

// use hickory_proto::rr::Name;
// use hickory_server::authority::MessageResponseBuilder;
use crate::ConfigResolver;

#[derive(Clone, Debug)]
pub struct DNSServer {
    pub upstream: ConfigResolver,
    pub blocking_ip: Ipv4Addr,
}

pub static DEFAULT_BLOCKING_IP: Ipv4Addr = Ipv4Addr::new(23, 50, 77, 69);
impl Default for DNSServer {
    fn default() -> DNSServer {
        DNSServer { upstream: Default::default(), blocking_ip: DEFAULT_BLOCKING_IP }
    }
}
impl DNSServer {
    pub async fn start(self, socket: UdpSocket) -> Result<(), Report> {
        let mut server = Server::new(self);
        let socket_addr = socket.local_addr()?;
        server.register_socket(socket);
        server.block_until_done().await.map_err(|err| {
            Report::msg("Error running DNS Server")
                .note(format!("udp socket: {socket_addr}"))
                .warning("warning")
                .suggestion(
                    "check if there isn't another server already bound to \
                     {local_addr}",
                )
                .error(err)
        })?;
        Ok(())
    }
}

#[async_trait]
impl RequestHandler for DNSServer {
    async fn handle_request<R: ResponseHandler, T: hickory_server::net::runtime::Time>(
        &self,
        request: &Request,
        mut response_handler: R,
    ) -> ResponseInfo {
        // eprintln!("\x1b[1;38;2;193;255;153mhandling request\x1b[0m");
        let info = request.request_info().unwrap();
        let _mr: &MessageRequest = request.deref();
        // eprintln!("{} requests ip address of {}", info.src, info.query.name());
        let builder = MessageResponseBuilder::from_message_request(request);
        let blocking_ip = DEFAULT_BLOCKING_IP;
        let mut metadata = Metadata::response_from_request(info.metadata);
        metadata.authoritative = true;
        let _header = Header { metadata, counts: HeaderCounts::default() };

        let name = request.queries.queries()[0].name();
        let names_to_resolve = request
            .queries
            .queries()
            .iter()
            .map(|query| query.name().to_string())
            .collect::<Vec<String>>();

        let mut upstream = self.upstream.clone();
        match upstream.resolve(names_to_resolve.into_iter()).await {
            Ok(ipv4_addrs) => {
                let records: Vec<_> = ipv4_addrs
                    .clone()
                    .into_iter()
                    .enumerate()
                    .map(|(idx, ipv4)| (idx, ipv4.parse::<Ipv4Addr>()))
                    .filter(|(idx, result)| {
                        if let Err(error) = result {
                            // ipv6
                            false
                        } else {
                            true
                        }
                    })
                    .map(|(idx, addr)| RData::A(A::from(addr.unwrap())))
                    // .filter(|ip| ip.is_some())
                    // .map(|ip| ip.unwrap())
                    .map(|rdata| Record::from_rdata(name.into(), 60, rdata))
                    .collect();

                let response = builder.build(metadata, records.iter(), &[], &[], &[]);
                response_handler.send_response(response).await.unwrap()
            }
            Err(_error) => {
                metadata.response_code = ResponseCode::ServFail;
                let response = builder.build_no_records(metadata);
                response_handler.send_response(response).await.unwrap()
            }
        }
    }
}
