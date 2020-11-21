quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Bootstrap(message: String) {
            from()
            display("bootstrap error: {}", message)
        }
        Io(err: std::io::Error) {
            from()
            source(err)
            display("{}", err)
        }
        Rpc(status: tonic::Status) {
            from()
            display("{}", status)
        }
        Transport(err: tonic::transport::Error) {
            from()
            display("{}", err)
        }
        Other(err: Box<dyn std::error::Error>) {
            from()
            source(err.as_ref())
            display("{:?}", err)
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
