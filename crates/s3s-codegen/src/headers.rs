use crate::gen::Codegen;
use crate::smithy;
use crate::{default, f};

use std::collections::BTreeSet;

use heck::ToShoutySnakeCase;

pub fn codegen(model: &smithy::Model, g: &mut Codegen) {
    let mut headers: BTreeSet<&str> = default();

    for (name, shape) in &model.shapes {
        if name.ends_with("Request") || name.ends_with("Output") {
            let smithy::Shape::Structure(sh) = shape else { panic!() };

            for member in sh.members.values() {
                if let Some(header) = member.traits.http_header() {
                    headers.insert(header);
                }
            }
        }
    }

    {
        headers.insert("x-amz-content-sha256");
        headers.insert("x-amz-date");
        headers.insert("authorization");
        headers.insert("host");
    }

    let prelude = [
        "//! Auto generated header name definitions",
        "#![allow(clippy::declare_interior_mutable_const)]",
        "",
        "use hyper::header::HeaderName;",
        "",
    ];

    for line in prelude {
        g.ln(line);
    }

    for header in headers {
        let name = to_constant_name(header);
        if header.starts_with("x-amz-") || header == "Content-MD5" {
            let value = header.to_ascii_lowercase();
            g.ln(f!("pub const {name}: HeaderName = HeaderName::from_static({value:?});",));
        } else {
            g.ln(f!("pub use hyper::header::{name};"))
        }
        g.lf();
    }
}

pub fn to_constant_name(header_name: &str) -> String {
    if header_name == "ETag" {
        "ETAG".into()
    } else {
        header_name.to_shouty_snake_case()
    }
}
