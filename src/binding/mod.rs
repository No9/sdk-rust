//! Provides protocol binding implementations for [`crate::Event`].

#[cfg_attr(docsrs, doc(cfg(feature = "actix")))]
#[cfg(feature = "actix")]
pub mod actix;
#[cfg_attr(docsrs, doc(cfg(feature = "axum")))]
#[cfg(feature = "axum")]
pub mod axum;

// #[cfg_attr(
//     docsrs,
//     doc(cfg(any(
//         feature = "http-binding",
//     )))
// )]
// #[cfg(any(
//     feature = "http-binding",
// ))]
pub mod http;

pub(crate) static CLOUDEVENTS_JSON_HEADER: &str = "application/cloudevents+json";
pub(crate) static CLOUDEVENTS_BATCH_JSON_HEADER: &str = "application/cloudevents-batch+json";
pub(crate) static CONTENT_TYPE: &str = "content-type";

fn header_prefix(prefix: &str, name: &str) -> String {
    if name == "datacontenttype" {
        CONTENT_TYPE.to_string()
    } else {
        [prefix, name].concat()
    }
}

#[macro_export]
macro_rules! header_value_to_str {
    ($header_value:expr) => {
        $header_value
            .to_str()
            .map_err(|e| $crate::message::Error::Other {
                source: Box::new(e),
            })
    };
}
