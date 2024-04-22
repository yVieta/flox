use std::str::FromStr;

use async_trait::async_trait;
use catalog_api_v1::types::{self as api_types, error as api_error, PackageInfoApiInput};
use catalog_api_v1::{Client as APIClient, Error as APIError};
use enum_dispatch::enum_dispatch;
use thiserror::Error;

use crate::data::System;
use crate::models::search::{SearchResult, SearchResults};

pub const DEFAULT_CATALOG_URL: &str = "https://flox-catalog.flox.dev";
const NIXPKGS_CATALOG: &str = "nixpkgs";

/// Either a client for the actual catalog service,
/// or a mock client for testing.
#[derive(Debug)]
#[enum_dispatch(ClientTrait)]
pub enum Client {
    Catalog(CatalogClient),
    Mock(MockClient),
}

/// A client for the catalog service.
///
/// This is a wrapper around the auto-generated APIClient.
#[derive(Debug)]
pub struct CatalogClient {
    client: APIClient,
}
impl CatalogClient {
    pub fn new() -> Self {
        Self {
            client: APIClient::new(DEFAULT_CATALOG_URL),
        }
    }
}

#[derive(Debug)]
pub struct MockClient;

impl Default for CatalogClient {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
#[enum_dispatch]
pub trait ClientTrait {
    /// Resolve a list of [PackageGroup]s into a list of
    /// [ResolvedPackageGroup]s.
    async fn resolve(
        &self,
        package_groups: Vec<PackageGroup>,
    ) -> Result<Vec<ResolvedPackageGroup>, ResolveError>;

    /// Search for packages in the catalog that match a given search_term.
    async fn search(
        &self,
        search_term: impl AsRef<str> + Send + Sync,
        system: System,
        limit: u8,
    ) -> Result<SearchResults, SearchError>;
}

#[async_trait]
impl ClientTrait for CatalogClient {
    /// Wrapper around the autogenerated
    /// [catalog_api_v1::Client::resolve_api_v1_catalog_resolve_post]
    async fn resolve(
        &self,
        package_groups: Vec<PackageGroup>,
    ) -> Result<Vec<ResolvedPackageGroup>, ResolveError> {
        let package_groups = api_types::PackageGroups {
            items: package_groups
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
        };

        let response = self
            .client
            .resolve_api_v1_catalog_resolve_post(&package_groups)
            .await
            .map_err(|e| {
                if CatalogClientError::is_unexpected_error(&e) {
                    CatalogClientError::UnexpectedError(e).into()
                } else {
                    ResolveError::Resolve(e)
                }
            })?;

        let resolved_package_groups = response.into_inner();

        Ok(resolved_package_groups
            .items
            .into_iter()
            .map(TryInto::try_into)
            .collect::<Result<Vec<_>, _>>()?)
    }

    /// Wrapper around the autogenerated
    /// [catalog_api_v1::Client::search_api_v1_catalog_search_get]
    async fn search(
        &self,
        search_term: impl AsRef<str> + Send + Sync,
        system: System,
        limit: u8,
    ) -> Result<SearchResults, SearchError> {
        let response = self
            .client
            .search_api_v1_catalog_search_get(
                Some(NIXPKGS_CATALOG),
                None,
                Some(limit.into()),
                &api_types::SearchTerm::from_str(search_term.as_ref())
                    .map_err(SearchError::InvalidSearchTerm)?,
                system
                    .try_into()
                    .map_err(CatalogClientError::UnsupportedSystem)?,
            )
            .await
            .map_err(|e| {
                if CatalogClientError::is_unexpected_error(&e) {
                    CatalogClientError::UnexpectedError(e).into()
                } else {
                    SearchError::Search(e)
                }
            })?;

        let api_search_result = response.into_inner();
        let search_results = SearchResults {
            results: api_search_result
                .items
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
            count: Some(
                api_search_result
                    .total_count
                    .try_into()
                    .map_err(|_| SearchError::NegativeNumberOfResults)?,
            ),
        };
        Ok(search_results)
    }
}

#[async_trait]
impl ClientTrait for MockClient {
    async fn resolve(
        &self,
        _package_groups: Vec<PackageGroup>,
    ) -> Result<Vec<ResolvedPackageGroup>, ResolveError> {
        unimplemented!()
    }

    async fn search(
        &self,
        _search_term: impl AsRef<str> + Send + Sync,
        _system: System,
        _limit: u8,
    ) -> Result<SearchResults, SearchError> {
        unimplemented!()
    }
}

/// Just an alias until the auto-generated PackageDescriptor diverges from what
/// we need.
pub type PackageDescriptor = api_types::PackageDescriptor;

pub struct PackageGroup {
    pub descriptors: Vec<PackageDescriptor>,
    pub name: String,
    pub system: System,
}

#[derive(Debug, Error)]
pub enum CatalogClientError {
    #[error("system not supported by catalog")]
    UnsupportedSystem(#[source] api_error::ConversionError),
    // TODO: would be nicer if this contained a ResponseValue<api_types::ErrorResponse>,
    // but that doesn't implement the necessary traits.
    /// UnexpectedError corresponds to any variant of APIError other than
    /// ErrorResponse, which is the only error that is in the API schema.
    #[error("unexpected catalog connection error")]
    UnexpectedError(#[source] APIError<api_types::ErrorResponse>),
}

#[derive(Debug, Error)]
pub enum SearchError {
    // TODO: would be nicer if this contained a ResponseValue<api_types::ErrorResponse>,
    // but that doesn't implement the necessary traits.
    #[error("search failed")]
    Search(#[source] APIError<api_types::ErrorResponse>),
    #[error("negative number of search resuls")]
    NegativeNumberOfResults,
    #[error("invalid search term")]
    InvalidSearchTerm(#[source] api_error::ConversionError),
    #[error("encountered attribute path with less than 3 elements: {0}")]
    ShortAttributePath(String),
    #[error(transparent)]
    CatalogClientError(#[from] CatalogClientError),
}

#[derive(Debug, Error)]
pub enum ResolveError {
    #[error("resolution failed")]
    Resolve(#[source] APIError<api_types::ErrorResponse>),
    #[error(transparent)]
    CatalogClientError(#[from] CatalogClientError),
}

impl CatalogClientError {
    /// UnexpectedError corresponds to any variant of APIError other than
    /// ErrorResponse, which is the only error that is in the API schema.
    fn is_unexpected_error(error: &APIError<api_types::ErrorResponse>) -> bool {
        !matches!(error, APIError::ErrorResponse(_))
    }
}

impl TryFrom<PackageGroup> for api_types::PackageGroup {
    type Error = CatalogClientError;

    fn try_from(package_group: PackageGroup) -> Result<Self, CatalogClientError> {
        Ok(Self {
            descriptors: package_group.descriptors,
            name: package_group.name,
            system: package_group
                .system
                .try_into()
                .map_err(CatalogClientError::UnsupportedSystem)?,
            stability: None,
        })
    }
}

pub struct ResolvedPackageGroup {
    pub name: String,
    pub pages: Vec<CatalogPage>,
    pub system: System,
}

impl TryFrom<api_types::ResolvedPackageGroupInput> for ResolvedPackageGroup {
    type Error = CatalogClientError;

    fn try_from(
        resolved_package_group: api_types::ResolvedPackageGroupInput,
    ) -> Result<Self, CatalogClientError> {
        Ok(Self {
            name: resolved_package_group.name,
            pages: resolved_package_group
                .pages
                .into_iter()
                .map(Into::into)
                .collect::<Vec<_>>(),
            system: resolved_package_group.system.to_string(),
        })
    }
}

pub struct CatalogPage {
    pub packages: Vec<PackageResolutionInfo>,
    pub page: i64,
    pub url: String,
}

impl From<api_types::CatalogPage> for CatalogPage {
    fn from(catalog_page: api_types::CatalogPage) -> Self {
        Self {
            packages: catalog_page.packages,
            page: catalog_page.page,
            url: catalog_page.url,
        }
    }
}

/// TODO: fix types for outputs and outputs_to_install,
/// at which point this will probably no longer be an alias.
type PackageResolutionInfo = api_types::PackageResolutionInfo;

impl TryFrom<PackageInfoApiInput> for SearchResult {
    type Error = SearchError;

    fn try_from(package_info: PackageInfoApiInput) -> Result<Self, SearchError> {
        Ok(Self {
            input: NIXPKGS_CATALOG.to_string(),
            system: package_info.system.to_string(),
            // The server does not include legacyPackages.<system> in attr_path
            rel_path: package_info
                .attr_path
                .split('.')
                .map(String::from)
                .collect(),
            pname: Some(package_info.pname),
            version: Some(package_info.version),
            description: Some(package_info.description),
            license: Some(package_info.license),
        })
    }
}
