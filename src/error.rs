error_chain!{
    foreign_links {
        ReqwestError(reqwest::Error);
        RequestUrlError(reqwest::UrlError);
        IoError(std::io::Error);
        EnvError(std::env::VarError);
        InvalidMethodError(http::method::InvalidMethod);
    }

    errors {

    }
}