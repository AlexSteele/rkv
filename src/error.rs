quick_error! {
    #[derive(Debug)]
    pub enum Error {
        InvalidArgument(msg: String) {
            display("invalid argument: {}", msg)
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
        TooFewReplicas {}
        Other(err: Box<dyn std::error::Error + Send + Sync + 'static>) {
            from()
            source(err.as_ref())
            display("{:?}", err)
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
