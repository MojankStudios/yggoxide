use okapi::openapi3::{self, SecurityScheme, SecuritySchemeData};
use rocket::{
    http::{ContentType, Status},
    request::{FromParam, FromRequest, Outcome},
    response::{self, Responder},
    Request, Response,
};
use rocket_okapi::{
    gen::OpenApiGenerator,
    request::{OpenApiFromRequest, RequestHeaderInput},
    response::OpenApiResponderInner,
};
use schemars::schema::{InstanceType, Metadata, Schema, SchemaObject, SingleOrVec};
use schemars::JsonSchema;

use crate::{
    structs::{common::Uuid, services::AccessToken},
    Error,
};

/// HTTP response builder for Error enum
impl<'r> Responder<'r, 'static> for Error {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        let status = match self {
            _ => Status::BadRequest,
        };

        // Serialize the error data structure into JSON.
        let string = json!(self).to_string();

        // Build and send the request.
        Response::build()
            .sized_body(string.len(), std::io::Cursor::new(string))
            .header(ContentType::new("application", "json"))
            .status(status)
            .ok()
    }
}

impl OpenApiResponderInner for Error {
    fn responses(
        gen: &mut OpenApiGenerator,
    ) -> std::result::Result<openapi3::Responses, rocket_okapi::OpenApiError> {
        let mut content = okapi::Map::new();

        let settings = schemars::gen::SchemaSettings::default().with(|s| {
            s.option_nullable = true;
            s.option_add_null_type = false;
            s.definitions_path = "#/components/schemas/".to_string();
        });

        let mut schema_generator = settings.into_generator();
        let schema = schema_generator.root_schema_for::<Error>();

        let definitions = gen.schema_generator().definitions_mut();
        for (key, value) in schema.definitions {
            definitions.insert(key, value);
        }

        definitions.insert(
            "Error".to_string(),
            schemars::schema::Schema::Object(schema.schema),
        );

        content.insert(
            "application/json".to_string(),
            openapi3::MediaType {
                schema: Some(schemars::schema::SchemaObject {
                    reference: Some("#/components/schemas/Error".to_string()),
                    ..Default::default()
                }),
                ..Default::default()
            },
        );

        Ok(openapi3::Responses {
            default: Some(openapi3::RefOr::Object(openapi3::Response {
                content,
                description: "An error occurred.".to_string(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AccessToken {
    type Error = Error;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        if let Some(authorization) = request
            .headers()
            .get("authorization")
            .next()
            .map(|x| x.to_string())
        {
            if authorization.starts_with("Bearer ") {
                Outcome::Success(AccessToken(authorization.chars().skip(7).collect()))
            } else {
                Outcome::Failure((
                    Status::BadRequest,
                    Error::ForbiddenOperationException(Default::default()),
                ))
            }
        } else {
            Outcome::Failure((
                Status::Unauthorized,
                Error::ForbiddenOperationException(Default::default()),
            ))
        }
    }
}

impl<'r> OpenApiFromRequest<'r> for AccessToken {
    fn from_request_input(
        _gen: &mut OpenApiGenerator,
        _name: String,
        _required: bool,
    ) -> rocket_okapi::Result<RequestHeaderInput> {
        let mut requirements = schemars::Map::new();
        requirements.insert("Access Token".to_string(), vec![]);

        Ok(RequestHeaderInput::Security(
            "Access Token".to_string(),
            SecurityScheme {
                data: SecuritySchemeData::ApiKey {
                    name: "Authorization".to_string(),
                    location: "header".to_owned(),
                },
                description: None,
                extensions: schemars::Map::new(),
            },
            requirements,
        ))
    }
}

impl<'r> FromParam<'r> for Uuid {
    type Error = &'r str;

    fn from_param(param: &'r str) -> Result<Self, Self::Error> {
        uuid::Uuid::parse_str(param).map(Uuid).map_err(|_| param)
    }
}

impl JsonSchema for Uuid {
    fn schema_name() -> String {
        "UUID".to_owned()
    }

    fn json_schema(_gen: &mut schemars::gen::SchemaGenerator) -> Schema {
        Schema::Object(SchemaObject {
            metadata: Some(Box::new(Metadata {
                description: Some("Universally unique identifier".to_owned()),
                examples: vec![json!("3e997354-66f2-42eb-95a4-eb04a72ce7ea")],
                ..Default::default()
            })),
            format: Some("uuid".to_owned()),
            instance_type: Some(SingleOrVec::Single(Box::new(InstanceType::String))),
            ..Default::default()
        })
    }
}
