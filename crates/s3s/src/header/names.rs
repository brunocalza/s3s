//! Auto generated header name definitions
#![allow(clippy::declare_interior_mutable_const)]

use hyper::header::HeaderName;

pub use hyper::header::CACHE_CONTROL;

pub use hyper::header::CONTENT_DISPOSITION;

pub use hyper::header::CONTENT_ENCODING;

pub use hyper::header::CONTENT_LANGUAGE;

pub use hyper::header::CONTENT_LENGTH;

pub const CONTENT_MD5: HeaderName = HeaderName::from_static("content-md5");

pub use hyper::header::CONTENT_RANGE;

pub use hyper::header::CONTENT_TYPE;

pub use hyper::header::ETAG;

pub use hyper::header::EXPIRES;

pub use hyper::header::IF_MATCH;

pub use hyper::header::IF_MODIFIED_SINCE;

pub use hyper::header::IF_NONE_MATCH;

pub use hyper::header::IF_UNMODIFIED_SINCE;

pub use hyper::header::LAST_MODIFIED;

pub use hyper::header::LOCATION;

pub use hyper::header::RANGE;

pub use hyper::header::ACCEPT_RANGES;

pub use hyper::header::AUTHORIZATION;

pub use hyper::header::HOST;

pub const X_AMZ_ABORT_DATE: HeaderName = HeaderName::from_static("x-amz-abort-date");

pub const X_AMZ_ABORT_RULE_ID: HeaderName = HeaderName::from_static("x-amz-abort-rule-id");

pub const X_AMZ_ACL: HeaderName = HeaderName::from_static("x-amz-acl");

pub const X_AMZ_ARCHIVE_STATUS: HeaderName = HeaderName::from_static("x-amz-archive-status");

pub const X_AMZ_BUCKET_OBJECT_LOCK_ENABLED: HeaderName = HeaderName::from_static("x-amz-bucket-object-lock-enabled");

pub const X_AMZ_BUCKET_OBJECT_LOCK_TOKEN: HeaderName = HeaderName::from_static("x-amz-bucket-object-lock-token");

pub const X_AMZ_BYPASS_GOVERNANCE_RETENTION: HeaderName = HeaderName::from_static("x-amz-bypass-governance-retention");

pub const X_AMZ_CHECKSUM_ALGORITHM: HeaderName = HeaderName::from_static("x-amz-checksum-algorithm");

pub const X_AMZ_CHECKSUM_CRC32: HeaderName = HeaderName::from_static("x-amz-checksum-crc32");

pub const X_AMZ_CHECKSUM_CRC32C: HeaderName = HeaderName::from_static("x-amz-checksum-crc32c");

pub const X_AMZ_CHECKSUM_MODE: HeaderName = HeaderName::from_static("x-amz-checksum-mode");

pub const X_AMZ_CHECKSUM_SHA1: HeaderName = HeaderName::from_static("x-amz-checksum-sha1");

pub const X_AMZ_CHECKSUM_SHA256: HeaderName = HeaderName::from_static("x-amz-checksum-sha256");

pub const X_AMZ_CONFIRM_REMOVE_SELF_BUCKET_ACCESS: HeaderName =
    HeaderName::from_static("x-amz-confirm-remove-self-bucket-access");

pub const X_AMZ_CONTENT_SHA256: HeaderName = HeaderName::from_static("x-amz-content-sha256");

pub const X_AMZ_COPY_SOURCE: HeaderName = HeaderName::from_static("x-amz-copy-source");

pub const X_AMZ_COPY_SOURCE_IF_MATCH: HeaderName = HeaderName::from_static("x-amz-copy-source-if-match");

pub const X_AMZ_COPY_SOURCE_IF_MODIFIED_SINCE: HeaderName = HeaderName::from_static("x-amz-copy-source-if-modified-since");

pub const X_AMZ_COPY_SOURCE_IF_NONE_MATCH: HeaderName = HeaderName::from_static("x-amz-copy-source-if-none-match");

pub const X_AMZ_COPY_SOURCE_IF_UNMODIFIED_SINCE: HeaderName = HeaderName::from_static("x-amz-copy-source-if-unmodified-since");

pub const X_AMZ_COPY_SOURCE_RANGE: HeaderName = HeaderName::from_static("x-amz-copy-source-range");

pub const X_AMZ_COPY_SOURCE_SERVER_SIDE_ENCRYPTION_CUSTOMER_ALGORITHM: HeaderName =
    HeaderName::from_static("x-amz-copy-source-server-side-encryption-customer-algorithm");

pub const X_AMZ_COPY_SOURCE_SERVER_SIDE_ENCRYPTION_CUSTOMER_KEY: HeaderName =
    HeaderName::from_static("x-amz-copy-source-server-side-encryption-customer-key");

pub const X_AMZ_COPY_SOURCE_SERVER_SIDE_ENCRYPTION_CUSTOMER_KEY_MD5: HeaderName =
    HeaderName::from_static("x-amz-copy-source-server-side-encryption-customer-key-md5");

pub const X_AMZ_COPY_SOURCE_VERSION_ID: HeaderName = HeaderName::from_static("x-amz-copy-source-version-id");

pub const X_AMZ_DATE: HeaderName = HeaderName::from_static("x-amz-date");

pub const X_AMZ_DELETE_MARKER: HeaderName = HeaderName::from_static("x-amz-delete-marker");

pub const X_AMZ_EXPECTED_BUCKET_OWNER: HeaderName = HeaderName::from_static("x-amz-expected-bucket-owner");

pub const X_AMZ_EXPIRATION: HeaderName = HeaderName::from_static("x-amz-expiration");

pub const X_AMZ_FWD_ERROR_CODE: HeaderName = HeaderName::from_static("x-amz-fwd-error-code");

pub const X_AMZ_FWD_ERROR_MESSAGE: HeaderName = HeaderName::from_static("x-amz-fwd-error-message");

pub const X_AMZ_FWD_HEADER_CACHE_CONTROL: HeaderName = HeaderName::from_static("x-amz-fwd-header-cache-control");

pub const X_AMZ_FWD_HEADER_CONTENT_DISPOSITION: HeaderName = HeaderName::from_static("x-amz-fwd-header-content-disposition");

pub const X_AMZ_FWD_HEADER_CONTENT_ENCODING: HeaderName = HeaderName::from_static("x-amz-fwd-header-content-encoding");

pub const X_AMZ_FWD_HEADER_CONTENT_LANGUAGE: HeaderName = HeaderName::from_static("x-amz-fwd-header-content-language");

pub const X_AMZ_FWD_HEADER_CONTENT_RANGE: HeaderName = HeaderName::from_static("x-amz-fwd-header-content-range");

pub const X_AMZ_FWD_HEADER_CONTENT_TYPE: HeaderName = HeaderName::from_static("x-amz-fwd-header-content-type");

pub const X_AMZ_FWD_HEADER_E_TAG: HeaderName = HeaderName::from_static("x-amz-fwd-header-etag");

pub const X_AMZ_FWD_HEADER_EXPIRES: HeaderName = HeaderName::from_static("x-amz-fwd-header-expires");

pub const X_AMZ_FWD_HEADER_LAST_MODIFIED: HeaderName = HeaderName::from_static("x-amz-fwd-header-last-modified");

pub const X_AMZ_FWD_HEADER_ACCEPT_RANGES: HeaderName = HeaderName::from_static("x-amz-fwd-header-accept-ranges");

pub const X_AMZ_FWD_HEADER_X_AMZ_CHECKSUM_CRC32: HeaderName = HeaderName::from_static("x-amz-fwd-header-x-amz-checksum-crc32");

pub const X_AMZ_FWD_HEADER_X_AMZ_CHECKSUM_CRC32C: HeaderName = HeaderName::from_static("x-amz-fwd-header-x-amz-checksum-crc32c");

pub const X_AMZ_FWD_HEADER_X_AMZ_CHECKSUM_SHA1: HeaderName = HeaderName::from_static("x-amz-fwd-header-x-amz-checksum-sha1");

pub const X_AMZ_FWD_HEADER_X_AMZ_CHECKSUM_SHA256: HeaderName = HeaderName::from_static("x-amz-fwd-header-x-amz-checksum-sha256");

pub const X_AMZ_FWD_HEADER_X_AMZ_DELETE_MARKER: HeaderName = HeaderName::from_static("x-amz-fwd-header-x-amz-delete-marker");

pub const X_AMZ_FWD_HEADER_X_AMZ_EXPIRATION: HeaderName = HeaderName::from_static("x-amz-fwd-header-x-amz-expiration");

pub const X_AMZ_FWD_HEADER_X_AMZ_MISSING_META: HeaderName = HeaderName::from_static("x-amz-fwd-header-x-amz-missing-meta");

pub const X_AMZ_FWD_HEADER_X_AMZ_MP_PARTS_COUNT: HeaderName = HeaderName::from_static("x-amz-fwd-header-x-amz-mp-parts-count");

pub const X_AMZ_FWD_HEADER_X_AMZ_OBJECT_LOCK_LEGAL_HOLD: HeaderName =
    HeaderName::from_static("x-amz-fwd-header-x-amz-object-lock-legal-hold");

pub const X_AMZ_FWD_HEADER_X_AMZ_OBJECT_LOCK_MODE: HeaderName =
    HeaderName::from_static("x-amz-fwd-header-x-amz-object-lock-mode");

pub const X_AMZ_FWD_HEADER_X_AMZ_OBJECT_LOCK_RETAIN_UNTIL_DATE: HeaderName =
    HeaderName::from_static("x-amz-fwd-header-x-amz-object-lock-retain-until-date");

pub const X_AMZ_FWD_HEADER_X_AMZ_REPLICATION_STATUS: HeaderName =
    HeaderName::from_static("x-amz-fwd-header-x-amz-replication-status");

pub const X_AMZ_FWD_HEADER_X_AMZ_REQUEST_CHARGED: HeaderName = HeaderName::from_static("x-amz-fwd-header-x-amz-request-charged");

pub const X_AMZ_FWD_HEADER_X_AMZ_RESTORE: HeaderName = HeaderName::from_static("x-amz-fwd-header-x-amz-restore");

pub const X_AMZ_FWD_HEADER_X_AMZ_SERVER_SIDE_ENCRYPTION: HeaderName =
    HeaderName::from_static("x-amz-fwd-header-x-amz-server-side-encryption");

pub const X_AMZ_FWD_HEADER_X_AMZ_SERVER_SIDE_ENCRYPTION_AWS_KMS_KEY_ID: HeaderName =
    HeaderName::from_static("x-amz-fwd-header-x-amz-server-side-encryption-aws-kms-key-id");

pub const X_AMZ_FWD_HEADER_X_AMZ_SERVER_SIDE_ENCRYPTION_BUCKET_KEY_ENABLED: HeaderName =
    HeaderName::from_static("x-amz-fwd-header-x-amz-server-side-encryption-bucket-key-enabled");

pub const X_AMZ_FWD_HEADER_X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_ALGORITHM: HeaderName =
    HeaderName::from_static("x-amz-fwd-header-x-amz-server-side-encryption-customer-algorithm");

pub const X_AMZ_FWD_HEADER_X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_KEY_MD5: HeaderName =
    HeaderName::from_static("x-amz-fwd-header-x-amz-server-side-encryption-customer-key-md5");

pub const X_AMZ_FWD_HEADER_X_AMZ_STORAGE_CLASS: HeaderName = HeaderName::from_static("x-amz-fwd-header-x-amz-storage-class");

pub const X_AMZ_FWD_HEADER_X_AMZ_TAGGING_COUNT: HeaderName = HeaderName::from_static("x-amz-fwd-header-x-amz-tagging-count");

pub const X_AMZ_FWD_HEADER_X_AMZ_VERSION_ID: HeaderName = HeaderName::from_static("x-amz-fwd-header-x-amz-version-id");

pub const X_AMZ_FWD_STATUS: HeaderName = HeaderName::from_static("x-amz-fwd-status");

pub const X_AMZ_GRANT_FULL_CONTROL: HeaderName = HeaderName::from_static("x-amz-grant-full-control");

pub const X_AMZ_GRANT_READ: HeaderName = HeaderName::from_static("x-amz-grant-read");

pub const X_AMZ_GRANT_READ_ACP: HeaderName = HeaderName::from_static("x-amz-grant-read-acp");

pub const X_AMZ_GRANT_WRITE: HeaderName = HeaderName::from_static("x-amz-grant-write");

pub const X_AMZ_GRANT_WRITE_ACP: HeaderName = HeaderName::from_static("x-amz-grant-write-acp");

pub const X_AMZ_MAX_PARTS: HeaderName = HeaderName::from_static("x-amz-max-parts");

pub const X_AMZ_METADATA_DIRECTIVE: HeaderName = HeaderName::from_static("x-amz-metadata-directive");

pub const X_AMZ_MFA: HeaderName = HeaderName::from_static("x-amz-mfa");

pub const X_AMZ_MISSING_META: HeaderName = HeaderName::from_static("x-amz-missing-meta");

pub const X_AMZ_MP_PARTS_COUNT: HeaderName = HeaderName::from_static("x-amz-mp-parts-count");

pub const X_AMZ_OBJECT_ATTRIBUTES: HeaderName = HeaderName::from_static("x-amz-object-attributes");

pub const X_AMZ_OBJECT_LOCK_LEGAL_HOLD: HeaderName = HeaderName::from_static("x-amz-object-lock-legal-hold");

pub const X_AMZ_OBJECT_LOCK_MODE: HeaderName = HeaderName::from_static("x-amz-object-lock-mode");

pub const X_AMZ_OBJECT_LOCK_RETAIN_UNTIL_DATE: HeaderName = HeaderName::from_static("x-amz-object-lock-retain-until-date");

pub const X_AMZ_OBJECT_OWNERSHIP: HeaderName = HeaderName::from_static("x-amz-object-ownership");

pub const X_AMZ_PART_NUMBER_MARKER: HeaderName = HeaderName::from_static("x-amz-part-number-marker");

pub const X_AMZ_REPLICATION_STATUS: HeaderName = HeaderName::from_static("x-amz-replication-status");

pub const X_AMZ_REQUEST_CHARGED: HeaderName = HeaderName::from_static("x-amz-request-charged");

pub const X_AMZ_REQUEST_PAYER: HeaderName = HeaderName::from_static("x-amz-request-payer");

pub const X_AMZ_REQUEST_ROUTE: HeaderName = HeaderName::from_static("x-amz-request-route");

pub const X_AMZ_REQUEST_TOKEN: HeaderName = HeaderName::from_static("x-amz-request-token");

pub const X_AMZ_RESTORE: HeaderName = HeaderName::from_static("x-amz-restore");

pub const X_AMZ_RESTORE_OUTPUT_PATH: HeaderName = HeaderName::from_static("x-amz-restore-output-path");

pub const X_AMZ_SDK_CHECKSUM_ALGORITHM: HeaderName = HeaderName::from_static("x-amz-sdk-checksum-algorithm");

pub const X_AMZ_SERVER_SIDE_ENCRYPTION: HeaderName = HeaderName::from_static("x-amz-server-side-encryption");

pub const X_AMZ_SERVER_SIDE_ENCRYPTION_AWS_KMS_KEY_ID: HeaderName =
    HeaderName::from_static("x-amz-server-side-encryption-aws-kms-key-id");

pub const X_AMZ_SERVER_SIDE_ENCRYPTION_BUCKET_KEY_ENABLED: HeaderName =
    HeaderName::from_static("x-amz-server-side-encryption-bucket-key-enabled");

pub const X_AMZ_SERVER_SIDE_ENCRYPTION_CONTEXT: HeaderName = HeaderName::from_static("x-amz-server-side-encryption-context");

pub const X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_ALGORITHM: HeaderName =
    HeaderName::from_static("x-amz-server-side-encryption-customer-algorithm");

pub const X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_KEY: HeaderName =
    HeaderName::from_static("x-amz-server-side-encryption-customer-key");

pub const X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_KEY_MD5: HeaderName =
    HeaderName::from_static("x-amz-server-side-encryption-customer-key-md5");

pub const X_AMZ_SKIP_DESTINATION_VALIDATION: HeaderName = HeaderName::from_static("x-amz-skip-destination-validation");

pub const X_AMZ_SOURCE_EXPECTED_BUCKET_OWNER: HeaderName = HeaderName::from_static("x-amz-source-expected-bucket-owner");

pub const X_AMZ_STORAGE_CLASS: HeaderName = HeaderName::from_static("x-amz-storage-class");

pub const X_AMZ_TAGGING: HeaderName = HeaderName::from_static("x-amz-tagging");

pub const X_AMZ_TAGGING_COUNT: HeaderName = HeaderName::from_static("x-amz-tagging-count");

pub const X_AMZ_TAGGING_DIRECTIVE: HeaderName = HeaderName::from_static("x-amz-tagging-directive");

pub const X_AMZ_VERSION_ID: HeaderName = HeaderName::from_static("x-amz-version-id");

pub const X_AMZ_WEBSITE_REDIRECT_LOCATION: HeaderName = HeaderName::from_static("x-amz-website-redirect-location");
