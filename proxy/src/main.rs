use async_trait::async_trait;
use pingora::prelude::*;
pub struct LB();

#[async_trait]
impl ProxyHttp for LB {
    type CTX = ();
    fn new_ctx(&self) -> () {
        ()
    }

    async fn upstream_peer(&self, _session: &mut Session, _ctx: &mut ()) -> Result<Box<HttpPeer>> {
        let addr: (&str, u16) = ("localhost", 3000);
        let peer: Box<HttpPeer> = Box::new(HttpPeer::new(addr, false, "".to_string()));
        Ok(peer)
    }
}

fn main() {
    let mut my_server = Server::new(None).unwrap();
    my_server.bootstrap();

    // Reverse Proxy Service
    let mut my_proxy: pingora::services::listening::Service<pingora::proxy::HttpProxy<LB>> = http_proxy_service(
        &my_server.configuration,
        LB {},
    );
    my_proxy.add_tcp("0.0.0.0:8081");
    
    my_server.add_service(my_proxy);
    my_server.run_forever();
}
