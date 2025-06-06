//! Trust anchor-related structures as defined in RFC 5914

use crate::certificate::{CertificateInner, Profile, Rfc5280, TbsCertificateInner};
use crate::ext::pkix::{NameConstraints, certpolicy::CertificatePolicies};
use crate::{ext::Extensions, name::Name};

use crate::SubjectPublicKeyInfo;
use alloc::string::String;
use der::{
    Choice, Enumerated, Sequence,
    asn1::OctetString,
    flagset::{FlagSet, flags},
};

/// Version identifier for TrustAnchorInfo
#[derive(Clone, Debug, Default, Copy, PartialEq, Eq, Enumerated)]
#[asn1(type = "INTEGER")]
#[repr(u8)]
pub enum Version {
    /// Version 1 (default)
    #[default]
    V1 = 0,
}

/// ```text
/// TrustAnchorInfo ::= SEQUENCE {
///     version         TrustAnchorInfoVersion DEFAULT v1,
///     pubKey          SubjectPublicKeyInfo,
///     keyId           KeyIdentifier,
///     taTitle         TrustAnchorTitle OPTIONAL,
///     certPath        CertPathControls OPTIONAL,
///     exts            [1] EXPLICIT Extensions   OPTIONAL,
///     taTitleLangTag  [2] UTF8String OPTIONAL
/// }
///
/// TrustAnchorInfoVersion ::= INTEGER { v1(1) }
///
/// TrustAnchorTitle ::= UTF8String (SIZE (1..64))
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Sequence)]
#[allow(missing_docs)]
pub struct TrustAnchorInfo<P: Profile = Rfc5280> {
    #[asn1(default = "Default::default")]
    pub version: Version,

    pub pub_key: SubjectPublicKeyInfo,

    pub key_id: OctetString,

    #[asn1(optional = "true")]
    pub ta_title: Option<String>,

    #[asn1(optional = "true")]
    pub cert_path: Option<CertPathControls<P>>,

    #[asn1(context_specific = "1", tag_mode = "EXPLICIT", optional = "true")]
    pub extensions: Option<Extensions>,

    #[asn1(context_specific = "2", tag_mode = "IMPLICIT", optional = "true")]
    pub ta_title_lang_tag: Option<String>,
}

/// ```text
/// CertPathControls ::= SEQUENCE {
///     taName              Name,
///     certificate         [0] Certificate OPTIONAL,
///     policySet           [1] CertificatePolicies OPTIONAL,
///     policyFlags         [2] CertPolicyFlags OPTIONAL,
///     nameConstr          [3] NameConstraints OPTIONAL,
///     pathLenConstraint   [4] INTEGER (0..MAX) OPTIONAL
/// }
/// ```
#[derive(Clone, Debug, Eq, PartialEq, Sequence)]
#[allow(missing_docs)]
pub struct CertPathControls<P: Profile = Rfc5280> {
    pub ta_name: Name,

    #[asn1(context_specific = "0", tag_mode = "IMPLICIT", optional = "true")]
    pub certificate: Option<CertificateInner<P>>,

    #[asn1(context_specific = "1", tag_mode = "IMPLICIT", optional = "true")]
    pub policy_set: Option<CertificatePolicies>,

    #[asn1(context_specific = "2", tag_mode = "IMPLICIT", optional = "true")]
    pub policy_flags: Option<CertPolicyFlags>,

    #[asn1(context_specific = "3", tag_mode = "IMPLICIT", optional = "true")]
    pub name_constr: Option<NameConstraints>,

    #[asn1(context_specific = "4", tag_mode = "IMPLICIT", optional = "true")]
    pub path_len_constraint: Option<u32>,
}

flags! {
    /// Certificate policies as defined in [RFC 5280 Section 4.2.1.13].
    ///
    /// ```text
    /// CertPolicyFlags ::= BIT STRING {
    ///     inhibitPolicyMapping    (0),
    ///     requireExplicitPolicy   (1),
    ///     inhibitAnyPolicy        (2)
    /// }
    /// ```
    ///
    /// [RFC 5280 Section 4.2.1.13]: https://datatracker.ietf.org/doc/html/rfc5280#section-4.2.1.13
    #[allow(missing_docs)]
    pub enum CertPolicies: u8 {
        InhibitPolicyMapping = 1 << 0,
        RequireExplicitPolicy = 1 << 1,
        InhibitAnyPolicy = 1 << 2,
    }
}

/// Certificate policy flags as defined in [RFC 5280 Section 4.2.1.13].
///
/// [RFC 5280 Section 4.2.1.13]: https://datatracker.ietf.org/doc/html/rfc5280#section-4.2.1.13
pub type CertPolicyFlags = FlagSet<CertPolicies>;

/// TrustAnchorInfo allows for the representation of a single trust anchor.
/// Defined in [RFC 5914 Section 3].
///
/// ```text
/// TrustAnchorChoice ::= CHOICE {
///   certificate  Certificate,
///   tbsCert      [1] EXPLICIT TBSCertificate,
///   taInfo       [2] EXPLICIT TrustAnchorInfo
/// }
/// ```
///
/// [RFC 5914 Section 3]: https://www.rfc-editor.org/rfc/rfc5914#section-3
#[derive(Clone, Debug, PartialEq, Eq, Choice)]
#[allow(clippy::large_enum_variant)]
#[allow(missing_docs)]
pub enum TrustAnchorChoice<P: Profile = Rfc5280> {
    Certificate(CertificateInner<P>),

    #[asn1(context_specific = "1", tag_mode = "EXPLICIT", constructed = "true")]
    TbsCertificate(TbsCertificateInner<P>),

    #[asn1(context_specific = "2", tag_mode = "EXPLICIT", constructed = "true")]
    TaInfo(TrustAnchorInfo<P>),
}
