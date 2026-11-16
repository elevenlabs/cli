use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct BillingClient {
    pub http_client: HttpClient,
}

impl BillingClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// List the invoices charged to the reseller's card, newest first, one per workspace invoice
    ///
    /// # Arguments
    ///
    /// * `cursor` - Pagination cursor from previous response
    /// * `page_size` - Number of invoices per page
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use elevenlabs_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ElevenlabsClient::new(config).expect("Failed to build client");
    ///     client
    ///         .reception
    ///         .billing
    ///         .list_invoices(
    ///             &ListInvoicesQueryRequest {
    ///                 cursor: Some("cursor".to_string()),
    ///                 page_size: Some(1),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_invoices(
        &self,
        request: &ListInvoicesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ResellerInvoicesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/reception/reseller/billing/invoices",
                None,
                QueryBuilder::new()
                    .string("cursor", request.cursor.clone())
                    .int("page_size", request.page_size.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get one invoice charged to the reseller's card, broken down into the workspace invoice's lines
    ///
    /// # Arguments
    ///
    /// * `invoice_id` - ID of the invoice. List them with `GET /v1/reception/reseller/billing/invoices`.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use elevenlabs_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ElevenlabsClient::new(config).expect("Failed to build client");
    ///     client
    ///         .reception
    ///         .billing
    ///         .get_invoice(&"in_1QzX8vLkdIwHu7ix3Bv0Wy2d".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_invoice(
        &self,
        invoice_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ResellerInvoiceDetail, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/reception/reseller/billing/invoices/{}", invoice_id),
                None,
                None,
                options,
            )
            .await
    }

    /// List the next workspace renewals the reseller's card will be charged for, soonest first
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use elevenlabs_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ElevenlabsClient::new(config).expect("Failed to build client");
    ///     client.reception.billing.list_upcoming_payments(None).await;
    /// }
    /// ```
    pub async fn list_upcoming_payments(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<ResellerUpcomingPaymentsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/reception/reseller/billing/upcoming",
                None,
                None,
                options,
            )
            .await
    }
}
