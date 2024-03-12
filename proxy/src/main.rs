use async_trait::async_trait;
use pingora::http::ResponseHeader;
use pingora::prelude::*;
use pingora::prelude::http_proxy_service;
use std::sync::Arc;

pub struct LB(Arc<LoadBalancer<RoundRobin>>);

pub struct MyGateway {}

#[async_trait]
impl ProxyHttp for MyGateway {
    /// For this small example, we don't need context storage
    type CTX = ();
    fn new_ctx(&self) -> () {
        ()
    }

    async fn request_filter(&self, session: &mut Session, _ctx: &mut Self::CTX) -> Result<bool> {
        if !session.req_header().uri.path().starts_with("/frontend")
        {
            let _ = session.respond_error(403).await;
            // true: early return as the response is already written
            return Ok(true);
        }
        Ok(false)
    }

    async fn upstream_peer(&self, _session: &mut Session, _ctx: &mut ()) -> Result<Box<HttpPeer>> {
        let addr: (&str, u16) = if _session.req_header().uri.path().starts_with("/frontend") {
            ("127.0.0.1", 3000u16)
        } else {
            ("localhost", 3000u16)
        };
        // let addr: (&str, u16) =("frontend:3000", 443u16);

        println!("connecting to {addr:?}");

        let peer: Box<HttpPeer> = Box::new(HttpPeer::new(addr, true, "127.0.0.1".to_string()));
        Ok(peer)
    }

    async fn response_filter(
        &self,
        _session: &mut Session,
        upstream_response: &mut ResponseHeader,
        _ctx: &mut Self::CTX,
    ) -> Result<()>
    where
        Self::CTX: Send + Sync,
    {
        // replace existing header if any
        upstream_response
            .insert_header("Server", "MyGateway")
            .unwrap();
        // because we don't support h3
        upstream_response.remove_header("alt-svc");

        Ok(())
    }

}

fn main() {
    let mut my_server: Server = Server::new(None).unwrap();
    my_server.bootstrap();

    // Reverse Proxy Service
    let mut my_proxy: pingora::services::listening::Service<pingora::proxy::HttpProxy<MyGateway>> = http_proxy_service(
        &my_server.configuration,
        MyGateway {},
    );
    my_proxy.add_tcp("0.0.0.0:8081");
    
    my_server.add_service(my_proxy);
    my_server.run_forever();
}
