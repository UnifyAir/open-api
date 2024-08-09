use async_trait::async_trait;
use axum::extract::*;
use axum_extra::extract::{CookieJar, Multipart};
use bytes::Bytes;
use http::Method;
use serde::{Deserialize, Serialize};

use crate::{models, types::*};

#[derive(Debug, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum DeliverResponse {
    /// successful transfering of Delivery
    Status204_SuccessfulTransferingOfDelivery
    ,
    /// Temporary Redirect
    Status307_TemporaryRedirect
    {
        body: models::RedirectResponse,
        location:
            String
        ,
        param_3gpp_sbi_target_nf_id:
            Option<
                String
            >,
    }
    ,
    /// Permanent Redirect
    Status308_PermanentRedirect
    {
        body: models::RedirectResponse,
        location:
            String
        ,
        param_3gpp_sbi_target_nf_id:
            Option<
                String
            >,
    }
    ,
    /// Bad request
    Status400_BadRequest
    (models::ExtProblemDetails)
    ,
    /// Unauthorized
    Status401_Unauthorized
    (models::ExtProblemDetails)
    ,
    /// Forbidden
    Status403_Forbidden
    (models::ExtProblemDetails)
    ,
    /// Not Found
    Status404_NotFound
    (models::ExtProblemDetails)
    ,
    /// Length Required
    Status411_LengthRequired
    (models::ProblemDetails)
    ,
    /// Payload Too Large
    Status413_PayloadTooLarge
    (models::ExtProblemDetails)
    ,
    /// Unsupported Media Type
    Status415_UnsupportedMediaType
    (models::ExtProblemDetails)
    ,
    /// Too Many Requests
    Status429_TooManyRequests
    (models::ExtProblemDetails)
    ,
    /// Internal Server Error
    Status500_InternalServerError
    (models::ExtProblemDetails)
    ,
    /// Service Unavailable
    Status503_ServiceUnavailable
    (models::ExtProblemDetails)
    ,
    /// unsuccessful delivery of mobile terminated data - gateway timeout
    Status504_UnsuccessfulDeliveryOfMobileTerminatedData
    (models::DeliverError)
    ,
    /// Generic Error
    Status0_GenericError,
}


/// IndividualPduSessionNsmfNidd
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait IndividualPduSessionNsmfNidd {
    /// Delivery Service Operation.
    ///
    /// Deliver - POST /nsmf-pdusession/v1/nsmf-nidd/v1/pdu-sessions/{pduSessionRef}/deliver
    async fn deliver(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::DeliverPathParams,
        body: axum::body::Body,
    ) -> Result<DeliverResponse, String>;
}
