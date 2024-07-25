use std::{borrow::Cow, fmt::Display};

use actix_web::{error::JsonPayloadError, HttpResponse, ResponseError};
use http::StatusCode;
use http_api_problem::{HttpApiProblem, PROBLEM_JSON_MEDIA_TYPE};
use log::{error, warn};
use paperclip::actix::api_v2_errors;
use serde_json::{to_value, Value};
use sqlx::{postgres::PgDatabaseError, Error};
use strum::EnumIter;


#[api_v2_errors(code = 403, code = 500, code = 400, code = 404, code = 409)]
#[derive(Debug, Clone, EnumIter, strum::Display)]
pub enum MyProblem {
    // Functionnal errors
    PasswordTooShort(u8),
    EmailNotVerified,

    // Auth errors
    AuthFailedLogin,
    AuthFailedRefresh,
    AuthInvalidBiscuit,
    AuthBiscuitLookupError,
    AuthInvalidAuthorizationHeader,
    AuthNoAuthorizationHeader,
    AuthEmailExpired,

    // Generics errors
    Validation(validator::ValidationErrors),
    NotFound,
    InternalServerError,
    Forbidden,
    BadRequest,
}

impl From<sqlx::Error> for MyProblem {
    fn from(e: Error) -> Self {
        match e {
            Error::RowNotFound => MyProblem::NotFound,
            Error::Database(ex) => {
                // Goal map Box<dyn DatabaseError> to PgDatabaseError
                let pg_error: &PgDatabaseError = ex.try_downcast_ref::<PgDatabaseError>().unwrap();

                //let pg_error: PgDatabaseError = ex.into();

                match pg_error.constraint() {
                    _ => {
                        error!("Database error: {}", &pg_error);
                        MyProblem::InternalServerError
                    }
                }
            }
            err => {
                error!("{}", &err);
                MyProblem::InternalServerError
            }
        }
    }
}

impl From<lettre::error::Error> for MyProblem {
    fn from(err: lettre::error::Error) -> MyProblem {
        warn!("{err}");
        MyProblem::InternalServerError
    }
}

impl From<lettre::transport::smtp::Error> for MyProblem {
    fn from(err: lettre::transport::smtp::Error) -> MyProblem {
        warn!("{err}");
        MyProblem::InternalServerError
    }
}

impl From<mrml::prelude::parser::Error> for MyProblem {
    fn from(err: mrml::prelude::parser::Error) -> MyProblem {
        warn!("{err}");
        MyProblem::InternalServerError
    }
}

impl From<mrml::prelude::render::Error> for MyProblem {
    fn from(err: mrml::prelude::render::Error) -> MyProblem {
        warn!("{err}");
        MyProblem::InternalServerError
    }
}

impl From<MyProblem> for HttpApiProblem {
    fn from(my_problem: MyProblem) -> Self {
        let problem: Problem = my_problem.to_owned().into();
        HttpApiProblem::new(problem.status)
            .type_url(format!(
                "https://admin.kingfight.eu/documentation/errors/{my_problem}",
            )) // rely on Display trait of MyProblem
            .value("id".to_owned(), &my_problem.to_string()) // also rely on Display trait of MyProblem
            .value("validation".to_owned(), &problem.validation)
            .title(problem.title)
            .detail(problem.detail)
    }
}

impl ResponseError for MyProblem {
    fn status_code(&self) -> StatusCode {
        let problem: Problem = self.to_owned().into();
        problem.status
    }

    fn error_response(&self) -> HttpResponse {
        let problem: HttpApiProblem = self.to_owned().into();

        let effective_status = problem
            .status
            .unwrap_or(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR);
        let actix_status = actix_web::http::StatusCode::from_u16(effective_status.as_u16())
            .unwrap_or(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR);

        let json = problem.json_bytes();

        actix_web::HttpResponse::build(actix_status)
            .append_header((
                actix_web::http::header::CONTENT_TYPE,
                PROBLEM_JSON_MEDIA_TYPE,
            ))
            .body(json)
    }
}

#[derive(Debug, Clone)]
pub struct Problem {
    pub id: MyProblem,
    pub title: &'static str,
    pub detail: Cow<'static, str>,
    pub validation: Option<Value>,
    pub status: StatusCode,
}

impl From<MyProblem> for Problem {
    fn from(problem: MyProblem) -> Self {
        match problem {
            // Functionnal errors
            MyProblem::PasswordTooShort(min_length) => Problem {
                id: MyProblem::PasswordTooShort(min_length),
                title: "Le mot de passe est trop court",
                detail: format!("Le mot de passe doit comporter au moins {min_length} caractères.").into(),
                validation: None,
                status: StatusCode::UNPROCESSABLE_ENTITY,
            },
            MyProblem::EmailNotVerified => Problem {
                id: MyProblem::EmailNotVerified,
                title: "Email non vérifié",
                detail: "Vous devez vérifier votre adresse e-mail avant de pouvoir vous connecter.".into(),
                validation: None,
                status: StatusCode::FORBIDDEN,
            },


            // Auth errors
            MyProblem::AuthFailedLogin => Problem {
                id: MyProblem::AuthFailedLogin,
                title: "Échec de l'authentification",
                detail: "Les informations d'identification fournies ne sont pas valides.".into(),
                validation: None,
                status: StatusCode::FORBIDDEN,
            },
            MyProblem::AuthFailedRefresh => Problem {
                id: MyProblem::AuthFailedRefresh,
                title: "Échec de rafraîchissement du jeton d'accès",
                detail: "Le jeton de rafraîchissement fourni est probablement invalide ou expiré.".into(),
                validation: None,
                status: StatusCode::UNAUTHORIZED,
            },
            MyProblem::AuthInvalidBiscuit => Problem {
                id: MyProblem::AuthInvalidBiscuit,
                title: "Biscuit invalide",
                detail: "Le jeton d'authentification fourni (Biscuit) n'est pas valide, n'a pas été créé avec la clé privée actuelle ou a expiré.".into(),
                validation: None,
                status: StatusCode::FORBIDDEN,
            },
            MyProblem::AuthBiscuitLookupError => Problem {
                id: MyProblem::AuthBiscuitLookupError,
                title: "Impossible de vérifier dans la base de données si le Biscuit fourni a été révoqué",
                detail: "Cela est probablement dû à l'indisponibilité de la base de données.".into(),
                validation: None,
                status: StatusCode::INTERNAL_SERVER_ERROR,
            },
            MyProblem::AuthInvalidAuthorizationHeader => Problem {
                id: MyProblem::AuthInvalidAuthorizationHeader,
                title: "En-tête `Authorization` invalide",
                detail: "La valeur de l'en-tête `Authorization` n'a pas pu être décodée en tant que chaîne UTF-8 valide contenant `Bearer {UUID}`.".into(),
                validation: None,
                status: StatusCode::BAD_REQUEST,
            },
            MyProblem::AuthNoAuthorizationHeader => Problem {
                id: MyProblem::AuthNoAuthorizationHeader,
                title: "Aucun en-tête `Authorization` n'a été trouvé dans la requête HTTP",
                detail: "L'en-tête `Authorization` doit être fourni et doit contenir un jeton d'authentification.".into(),
                validation: None,
                status: StatusCode::UNAUTHORIZED,
            },
            MyProblem::AuthEmailExpired => {
                Problem {
                    id: MyProblem::AuthEmailExpired,
                    title: "Impossible de vérifier votre lien",
                    detail: "Le lien sur lequel vous avez cliqué pourrait avoir expiré. Veuillez réessayer tout le processus ou contacter le support.".into(),
                    validation: None,
                    status: StatusCode::UNAUTHORIZED,
                }
            },

            

            // Generics errors
            MyProblem::Validation(e) => {
                let errors_str = e.to_string();
                Problem {
                    id: MyProblem::Validation(e.to_owned()),
                    title: "L'entrée fournie est mal formée",
                    detail: errors_str.into(),
                    validation: to_value(e).ok(),
                    status: StatusCode::UNPROCESSABLE_ENTITY,
                }
            },
            MyProblem::NotFound => Problem {
                id: MyProblem::NotFound,
                title: "Ressource non trouvée",
                detail: "La ressource demandée est introuvable.".into(),
                validation: None,
                status: StatusCode::NOT_FOUND,
            },
            MyProblem::InternalServerError => Problem {
                id: MyProblem::InternalServerError,
                title: "Erreur interne du serveur",
                detail: "Une erreur inattendue s'est produite.".into(),
                validation: None,
                status: StatusCode::INTERNAL_SERVER_ERROR,
            },
            MyProblem::Forbidden => Problem {
                id: MyProblem::Forbidden,
                title: "Interdit",
                detail: "Vous n'avez pas la permission d'accéder à cette ressource.".into(),
                validation: None,
                status: StatusCode::FORBIDDEN,
            },
            MyProblem::BadRequest => Problem {
                id: MyProblem::BadRequest,
                title: "Mauvaise requête",
                detail: "La requête n'a pas pu être comprise par le serveur.".into(),
                validation: None,
                status: StatusCode::BAD_REQUEST,
            },
        }
    }
}

/// Simplified error type for the JSON body parser
#[derive(Debug, Clone)]
pub enum JsonPayloadProblem {
    Overflow { limit: usize },
    ContentType,
    Deserialize(String),
    Serialize(String),
    Payload(String),
    Other(String),
}

impl Default for JsonPayloadProblem {
    fn default() -> Self {
        Self::Other("".to_owned())
    }
}

impl Display for JsonPayloadProblem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Overflow { limit } => write!(f, "Body is too big (maximum is {limit} bytes)"),
            Self::ContentType => {
                write!(f, "Content-Type header should be set to 'application/json'")
            }
            Self::Deserialize(e) => write!(f, "JSON deserialization error: {e}"),
            Self::Serialize(e) => write!(f, "JSON serialization error: {e}"),
            Self::Payload(e) => write!(f, "Payload error: {e}"),
            Self::Other(e) => write!(f, "{e}"),
        }
    }
}

impl From<JsonPayloadError> for JsonPayloadProblem {
    fn from(e: JsonPayloadError) -> Self {
        match e {
            JsonPayloadError::OverflowKnownLength { length: _, limit } => Self::Overflow { limit },
            JsonPayloadError::Overflow { limit } => Self::Overflow { limit },
            JsonPayloadError::ContentType => Self::ContentType,
            JsonPayloadError::Deserialize(e) => Self::Deserialize(e.to_string()),
            JsonPayloadError::Serialize(e) => Self::Serialize(e.to_string()),
            JsonPayloadError::Payload(e) => Self::Payload(e.to_string()),
            e => Self::Other(e.to_string()),
        }
    }
}