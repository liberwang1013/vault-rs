error_chain! {
    foreign_links {
        ReqwestError(reqwest::Error);
        //ParseUrlError(reqwest::ParseError);
        IoError(std::io::Error);
        EnvError(std::env::VarError);
        InvalidMethodError(http::method::InvalidMethod);
    }

    errors {

    }
}
