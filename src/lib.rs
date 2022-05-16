pub type DateTime = String;
pub type Url = String;
pub type Text = String;
pub type SpdxLicenseExpression = String;
pub type Version = String;

/// metadata for an image
/// based on [oci image spec annotations](https://github.com/opencontainers/image-spec/blob/v1.0/annotations.md)
#[derive(Debug, ::serde::Serialize, ::serde::Deserialize)]
pub struct Annotations {
    /// the date and time on which the image was built (date-time string as defined by RFC 3339).
    #[serde(rename = "org.opencontainers.image.created")]
    pub created: Option<DateTime>,

    /// contact details of the people or organization responsible for the image (freeform string)
    #[serde(rename = "org.opencontainers.image.authors")]
    pub authors: Option<Text>,

    /// URL to find more information on the image
    #[serde(rename = "org.opencontainers.image.url")]
    pub url: Option<Url>,

    /// Human-readable description of the software packaged in the image
    #[serde(rename = "org.opencontainers.image.description")]
    pub description: Option<Text>,

    /// URL to get documentation on the image
    #[serde(rename = "org.opencontainers.image.documentation")]
    pub documentation: Option<Url>,

    /// version of the packaged software
    /// MAY match a label or tag in the source code repository
    /// MAY be Semantic versioning-compatible
    #[serde(rename = "org.opencontainers.image.version")]
    pub version: Option<Version>,

    /// License(s) under which contained software is distributed as an SPDX License Expression.
    #[serde(rename = "org.opencontainers.image.licenses")]
    pub licenses: Option<SpdxLicenseExpression>,

    /// URL to get source code for building the image
    #[serde(rename = "org.opencontainers.image.source")]
    pub source: Option<Url>,

    /// Name of the distributing entity, organization or individual.
    #[serde(rename = "org.opencontainers.image.vendor")]
    pub vendor: Option<Text>,

    /// Human-readable title of the image
    #[serde(rename = "org.opencontainers.image.title")]
    pub title: Option<Text>,
}
