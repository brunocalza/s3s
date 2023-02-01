use crate::gen::Codegen;
use crate::smithy;
use crate::{default, f, o};

use std::collections::BTreeMap;
use std::ops::Not;

use heck::ToShoutySnakeCase;
use regex::Regex;

struct Error {
    code: String,
    description: Vec<Option<String>>,
    status: Vec<Option<String>>,
}

pub fn codegen(model: &smithy::Model, g: &mut Codegen) {
    let error_code_doc = {
        let smithy::Shape::Structure(shape) = &model.shapes["com.amazonaws.s3#Error"] else { panic!() };
        shape.members["Code"].traits.doc().unwrap()
    };

    let pattern = Regex::new(r"<i>(.+?)</i> (.+)").unwrap();
    let code_pattern = Regex::new(r"<i>(.+?)</i> (.+?)</p>").unwrap();

    let mut errors: BTreeMap<String, Error> = default();

    let mut iter = error_code_doc.lines().map(|line| line.trim());
    while let Some(line) = iter.next() {
        let code = {
            let Some(cap) = pattern.captures(line) else { continue };
            let tag = cap.get(1).unwrap().as_str();
            assert_eq!(tag, "Code:");
            o(code_pattern.captures(line).unwrap().get(2).unwrap().as_str())
        };

        let description = loop {
            let Some(line) = iter.next() else { continue };
            let Some(cap) = pattern.captures(line) else { continue };
            let tag = cap.get(1).unwrap().as_str();
            if tag != "Description:" {
                break None;
            }
            let mut desc = String::new();
            let mut content = cap.get(2).unwrap().as_str();
            loop {
                match content.strip_suffix("</p>") {
                    Some(t) => {
                        if desc.is_empty().not() {
                            desc.push(' ');
                        }
                        desc.push_str(t);
                        break;
                    }
                    None => {
                        if desc.is_empty().not() {
                            desc.push(' ');
                        }
                        desc.push_str(content);
                        content = iter.next().unwrap();
                    }
                }
            }
            break Some(desc);
        };

        let status = loop {
            let Some(line) = iter.next() else { continue };

            if line.starts_with("<i>HTTP Status Code:</i> N/A") {
                break None;
            }

            if line.starts_with("<i>Code:</i> 409 Conflict") {
                break Some(o("409 Conflict"));
            }

            let Some(cap) = pattern.captures(line) else { continue };
            let tag = cap.get(1).unwrap().as_str();
            assert_eq!(tag, "HTTP Status Code:", "{line:?}");

            let mut status = String::new();
            let mut content = cap.get(2).unwrap().as_str();
            loop {
                match content.strip_suffix("</p>") {
                    Some(t) => {
                        status.push_str(t);
                        break;
                    }
                    None => {
                        status.push_str(content);
                        content = iter.next().unwrap();
                    }
                }
            }
            break Some(status);
        };

        let _ = loop {
            let Some(line) = iter.next() else { continue };
            let Some(cap) = pattern.captures(line) else { continue };
            break cap;
        };

        let err = errors.entry(code.clone()).or_insert_with(|| Error {
            code,
            description: default(),
            status: default(),
        });
        err.description.push(description);
        err.status.push(status);
    }

    g.lines([
        "use bytestring::ByteString;", //
        "use hyper::StatusCode;",      //
        "",                            //
    ]);

    g.ln("#[derive(Debug, Clone, PartialEq, Eq)]");
    g.ln("#[non_exhaustive]");
    g.ln("pub enum S3ErrorCode {");
    for err in errors.values() {
        if err.description.len() > 1 {
            assert_eq!(err.code, "InvalidRequest");
            for status in &err.status {
                assert_eq!(status.as_ref().unwrap(), "400 Bad Request");
            }
            for desc in &err.description {
                g.ln(f!("/// + {}", desc.as_ref().unwrap()));
            }
            g.ln("///");
            g.ln("/// HTTP Status Code: 400 Bad Request");
        } else {
            let desc = &err.description[0];
            let status = &err.status[0];

            if let Some(ref desc) = desc {
                g.ln(f!("/// {desc}"));
            }
            if let Some(ref status) = status {
                if desc.is_some() {
                    g.ln("///");
                }
                g.ln(f!("/// HTTP Status Code: {status}"));
            }
            if desc.is_some() || status.is_some() {
                g.ln("///");
            }
        }

        g.ln(f!("{},", err.code));
        g.lf();
    }
    g.ln("Custom(ByteString),");
    g.ln("}");
    g.lf();

    g.ln("impl S3ErrorCode {");

    {
        g.ln("#[must_use]");
        g.ln("pub fn as_str(&self) -> &str {");

        g.ln("match self {");
        for err in errors.values() {
            g.ln(f!("Self::{} => \"{}\",", err.code, err.code));
        }
        g.ln("Self::Custom(s) => s,");
        g.ln("}");

        g.ln("}");
        g.lf();
    }

    {
        g.ln("#[must_use]");
        g.ln("pub fn from_bytes(s: &[u8]) -> Option<Self> {");

        g.ln("match s {");
        for err in errors.values() {
            g.ln(f!("b\"{}\" => Some(Self::{}),", err.code, err.code));
        }
        g.ln("_ => std::str::from_utf8(s).ok().map(|s| Self::Custom(s.into()))");
        g.ln("}");

        g.ln("}");
        g.lf();
    }

    {
        g.ln("#[must_use]");
        g.ln("pub fn status_code(&self) -> Option<StatusCode> {");

        g.ln("match self {");
        for err in errors.values() {
            if err.status.len() > 1 {
                for status in &err.status {
                    assert_eq!(status.as_ref().unwrap(), "400 Bad Request");
                }
                g.ln(f!("Self::{} => Some(StatusCode::BAD_REQUEST),", err.code));
                continue;
            }
            if let Some(Some(status)) = err.status.first() {
                let status_name = match &status[4..] {
                    "Moved Temporarily" => {
                        assert!(status.starts_with("307"));
                        o("TEMPORARY_REDIRECT")
                    }
                    "Requested Range NotSatisfiable" => {
                        assert!(status.starts_with("416"));
                        o("RANGE_NOT_SATISFIABLE")
                    }
                    "Slow Down" => {
                        assert!(status.starts_with("503"));
                        o("SERVICE_UNAVAILABLE")
                    }
                    x => x.to_shouty_snake_case(),
                };

                g.ln(f!("Self::{} => Some(StatusCode::{}),", err.code, status_name));
                continue;
            }
            g.ln(f!("Self::{} => None,", err.code));
        }
        g.ln("Self::Custom(_) => None,");
        g.ln("}");

        g.ln("}");
        g.lf();
    }

    g.ln("}");
}
