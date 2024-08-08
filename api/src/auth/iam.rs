use biscuit_auth::{
    builder::Fact, builder_ext::AuthorizerExt, error, macros::*, AuthorizerLimits, Biscuit,
    KeyPair, PrivateKey,
};
use chrono::{DateTime, Utc};
use log::{error, trace};
use paperclip::v2::schema::TypedData;
use serde::Serialize;
use std::{
    time::{Duration, SystemTime},
    vec,
};
use strum::{AsRefStr, EnumIter, EnumString, IntoEnumIterator, VariantNames};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct RootToken {
    pub biscuit: Biscuit,
    pub serialized_biscuit: String,
    pub revocation_id: Vec<u8>,
    pub expired_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthorizedToken {
    User(AuthorizedUserToken),
    Service,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorizedUserToken {
    pub session_id: Uuid,
    pub user_id: Uuid,
    pub email: String,
    pub username: String,
    pub role: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorizedEmailVerificationToken {
    pub user_id: Uuid,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorizedResetPasswordToken {
    pub user_id: Uuid,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    strum::Display,
    EnumString,
    EnumIter,
    VariantNames,
    AsRefStr,
)]
#[strum(serialize_all = "snake_case")]
pub enum Role {
    Support,
    Moderateur,
    Responsable,
    Developpeur,
    Administrateur,
}

impl Default for Role {
    fn default() -> Self {
        Self::Support
    }
}

impl Role {
    pub fn values() -> Vec<String> {
        Role::iter().map(|r| r.to_string().to_lowercase()).collect()
    }

    pub fn get_order(&self) -> i32 {
        match self {
            Role::Support => 1,
            Role::Moderateur => 2,
            Role::Responsable => 3,
            Role::Developpeur => 4,
            Role::Administrateur => 5,
        }
    }

    pub fn to_role(role: &str) -> Role {
        match role {
            "support" => Role::Support,
            "moderateur" => Role::Moderateur,
            "responsable" => Role::Responsable,
            "developpeur" => Role::Developpeur,
            "administrateur" => Role::Administrateur,
            _ => Role::Moderateur,
        }
    }
}
impl TypedData for Role {
    fn data_type() -> paperclip::v2::models::DataType {
        paperclip::v2::models::DataType::String
    }

    fn format() -> Option<paperclip::v2::models::DataTypeFormat> {
        None
    }
}

impl Serialize for Role {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    AuthLogout,
    AuthChangePassword,
    UserSettingsChangeProfilePicture,
    UserSettingsChangeName,
    UserSettingsDeleteUser,
    StaffsGenerateRegistrationToken,
    StaffsGetUsers,
    StaffsSetRank,
    StaffsDeleteUser,
    StaffsGetLogs,
    EventsSendMessage,
    EventsIngest,
    EventsGetAll,
    BoutiqueGetLogs,
    BoutiqueGetPbLogs,
    ServiceAccessCreateServiceAccess,
    ServiceAccessDeleteServiceAccess,
    ServiceAccessGetServiceAccess,
    PlayersJoin,
    PlayersLeave,
    PlayersGetOnline,
    PlayersSanction,
    SanctionsCreate,
    SanctionsUpdate,
    SanctionsDelete,
    SanctionsGets,
    SanctionsGetPlayerSanction,
    SanctionsGetLogs,
    SanctionsMute,
    SanctionsKick,
    SanctionsBan,
    ErrorsGet,
    ErrorsPost,
}

impl<'a> Action {
    pub fn action_name(&self) -> &'static str {
        match self {
            Action::AuthLogout => "auth:logout",
            Action::AuthChangePassword => "auth:change_password",
            Action::UserSettingsChangeProfilePicture => "users-settings:change_profile_picture",
            Action::UserSettingsChangeName => "users-settings:change_name",
            Action::UserSettingsDeleteUser => "users-settings:delete_user",
            Action::StaffsGenerateRegistrationToken => {
                "users-management:generate_registration_token"
            }
            Action::StaffsGetUsers => "users-management:get-users",
            Action::StaffsSetRank => "users-management:set-role",
            Action::StaffsDeleteUser => "users-management:delete-user",
            Action::StaffsGetLogs => "users-management:get-logs",
            Action::EventsSendMessage => "users-management:send-message",
            Action::EventsIngest => "events:ingest",
            Action::EventsGetAll => "events:get_all",
            Action::BoutiqueGetLogs => "boutique:get-logs",
            Action::BoutiqueGetPbLogs => "boutique:get-pb-logs",
            Action::ServiceAccessCreateServiceAccess => "service-access:create-service-access",
            Action::ServiceAccessDeleteServiceAccess => "service-access:delete-service-access",
            Action::ServiceAccessGetServiceAccess => "service-access:get-service-access",
            Action::PlayersJoin => "players:join",
            Action::PlayersLeave => "players:leave",
            Action::PlayersGetOnline => "players:get-online",
            Action::PlayersSanction => "players:sanction",
            Action::SanctionsCreate => "sanctions:create",
            Action::SanctionsUpdate => "sanctions:update",
            Action::SanctionsDelete => "sanctions:delete",
            Action::SanctionsGets => "sanctions:gets",
            Action::SanctionsGetPlayerSanction => "sanctions:get-player-sanction",
            Action::SanctionsGetLogs => "sanctions:get-logs",
            Action::SanctionsMute => "sanctions:mute",
            Action::SanctionsKick => "sanctions:kick",
            Action::SanctionsBan => "sanctions:ban",
            Action::ErrorsGet => "errors:get",
            Action::ErrorsPost => "errors:post",
        }
    }

    fn allowed_roles(&self) -> Vec<Role> {
        let mut roles = vec![Role::Administrateur];
        let all_roles: Vec<Role> = vec![
            Role::Support,
            Role::Moderateur,
            Role::Responsable,
            Role::Developpeur,
        ];
        let head_staffs: Vec<Role> = vec![Role::Responsable, Role::Developpeur];

        let mut per_action_roles = match self {
            Self::AuthLogout => all_roles,
            Self::AuthChangePassword => all_roles,
            Self::UserSettingsChangeProfilePicture => all_roles,
            Self::UserSettingsChangeName => all_roles,
            Self::UserSettingsDeleteUser => all_roles,
            Self::StaffsGenerateRegistrationToken => vec![],
            Self::StaffsGetUsers => all_roles,
            Self::StaffsSetRank => head_staffs,
            Self::StaffsDeleteUser => head_staffs,
            Self::StaffsGetLogs => all_roles,
            Self::EventsSendMessage => all_roles,
            Self::EventsIngest => vec![],
            Self::EventsGetAll => vec![],
            Self::BoutiqueGetLogs => vec![],
            Self::BoutiqueGetPbLogs => vec![],
            Self::ServiceAccessCreateServiceAccess => vec![],
            Self::ServiceAccessDeleteServiceAccess => vec![],
            Self::ServiceAccessGetServiceAccess => vec![],
            Self::PlayersJoin => vec![],
            Self::PlayersLeave => vec![],
            Self::PlayersGetOnline => all_roles,
            Self::PlayersSanction => all_roles,
            Self::SanctionsCreate => head_staffs,
            Self::SanctionsUpdate => head_staffs,
            Self::SanctionsDelete => head_staffs,
            Self::SanctionsGets => all_roles,
            Self::SanctionsGetPlayerSanction => all_roles,
            Self::SanctionsGetLogs => all_roles,
            Self::SanctionsMute => all_roles,
            Self::SanctionsKick => vec![Role::Moderateur, Role::Responsable, Role::Developpeur],
            Self::SanctionsBan => vec![Role::Moderateur, Role::Responsable, Role::Developpeur],
            Self::ErrorsGet => head_staffs,
            Self::ErrorsPost => vec![],
        };

        roles.append(&mut per_action_roles);
        roles
    }

    pub fn generate_facts(self) -> Vec<Fact> {
        let mut facts = match self {
            Self::AuthLogout => vec![],
            Self::AuthChangePassword => vec![],
            Self::UserSettingsChangeProfilePicture => vec![],
            Self::UserSettingsChangeName => vec![],
            Self::UserSettingsDeleteUser => vec![],
            Self::StaffsGenerateRegistrationToken => vec![],
            Self::StaffsGetUsers => vec![],
            Self::StaffsSetRank => vec![],
            Self::StaffsDeleteUser => vec![],
            Self::StaffsGetLogs => vec![],
            Self::EventsSendMessage => vec![],
            Self::EventsIngest => vec![],
            Self::EventsGetAll => vec![],
            Self::BoutiqueGetLogs => vec![],
            Self::BoutiqueGetPbLogs => vec![],
            Self::ServiceAccessCreateServiceAccess => vec![],
            Self::ServiceAccessDeleteServiceAccess => vec![],
            Self::ServiceAccessGetServiceAccess => vec![],
            Self::PlayersJoin => vec![],
            Self::PlayersLeave => vec![],
            Self::PlayersGetOnline => vec![],
            Self::PlayersSanction => vec![],
            Self::SanctionsCreate => vec![],
            Self::SanctionsUpdate => vec![],
            Self::SanctionsDelete => vec![],
            Self::SanctionsGets => vec![],
            Self::SanctionsGetPlayerSanction => vec![],
            Self::SanctionsGetLogs => vec![],
            Self::SanctionsMute => vec![],
            Self::SanctionsKick => vec![],
            Self::SanctionsBan => vec![],
            Self::ErrorsGet => vec![],
            Self::ErrorsPost => vec![],
        };

        facts.push(fact!("action({action})", action = self.action_name()));

        for role in self.allowed_roles() {
            facts.push(fact!("allowed_role({role})", role = role.as_ref()));
        }

        facts
    }
}

const USER_ACCESS_TOKEN_VERSION: i64 = 1;
const USER_ACCESS_TOKEN_EXPIRATION: Duration = Duration::from_secs(60 * 5); // 5 minutes

pub fn create_user_access_token(
    private_key: &PrivateKey,
    token_id: Uuid,
    session_id: Uuid,
    user_id: Uuid,
    email: &str,
    username: &str,
    role: &str,
) -> Result<RootToken, biscuit_auth::error::Token> {
    let keypair = KeyPair::from(private_key);
    let created_at = SystemTime::now();
    let expired_at = created_at + USER_ACCESS_TOKEN_EXPIRATION;

    let biscuit = {
        let biscuit = biscuit!(
            r#"
                type("user_access");
                version({USER_ACCESS_TOKEN_VERSION});
                session_id({session_id});
                token_id({token_id});
                created_at({created_at});
                user_id({user_id});
                email({email});
                username({username});
                role({role});

                check if time($t), $t < {expired_at};
            "#,
        );
        biscuit.build(&keypair)?
    };
    let serialized_biscuit = biscuit.to_base64()?;
    let revocation_id = biscuit
        .revocation_identifiers()
        .first()
        .map(|rid| rid.to_owned())
        .ok_or(biscuit_auth::error::Token::InternalError)?;

    Ok(RootToken {
        biscuit,
        serialized_biscuit,
        revocation_id,
        expired_at: Some(DateTime::from(expired_at)),
    })
}

const REFRESH_TOKEN_VERSION: i64 = 1;
const REFRESH_TOKEN_EXPIRATION: Duration = Duration::from_secs(60 * 30);

pub fn create_refresh_token(
    private_key: &PrivateKey,
    token_id: Uuid,
    session_id: Uuid,
    user_id: Uuid,
) -> Result<RootToken, biscuit_auth::error::Token> {
    let keypair = KeyPair::from(private_key);
    let created_at = SystemTime::now();
    let expired_at = created_at + REFRESH_TOKEN_EXPIRATION;

    let biscuit = biscuit!(
        r#"
            type("refresh");
            version({REFRESH_TOKEN_VERSION});
            token_id({token_id});
            session_id({session_id});
            user_id({user_id});
            created_at({created_at});

            check if time($t), $t < {expired_at};
        "#,
    )
    .build(&keypair)?;
    let serialized_biscuit = biscuit.to_base64()?;
    let revocation_id = biscuit
        .revocation_identifiers()
        .first()
        .map(|rid| rid.to_owned())
        .ok_or(biscuit_auth::error::Token::InternalError)?;

    Ok(RootToken {
        biscuit,
        serialized_biscuit,
        revocation_id,
        expired_at: Some(DateTime::from(expired_at)),
    })
}

const EMAIL_VERIFICATION_TOKEN_VERSION: i64 = 1;
const EMAIL_VERIFICATION_TOKEN_EXPIRATION: Duration = Duration::from_secs(60 * 30);

pub fn create_email_verification_token(
    private_key: &PrivateKey,
    user_id: Uuid,
) -> Result<RootToken, biscuit_auth::error::Token> {
    let keypair = KeyPair::from(private_key);
    let created_at = SystemTime::now();

    let expired_at = created_at + EMAIL_VERIFICATION_TOKEN_EXPIRATION;

    let biscuit = biscuit!(
        r#"
            type("email_verification");
            version({EMAIL_VERIFICATION_TOKEN_VERSION});
            user_id({user_id});
            created_at({created_at});
            expired_at({expired_at});
        "#,
    )
    .build(&keypair)?;
    let serialized_biscuit = biscuit.to_base64()?;
    let revocation_id = biscuit
        .revocation_identifiers()
        .first()
        .map(|rid| rid.to_owned())
        .ok_or(biscuit_auth::error::Token::InternalError)?;

    Ok(RootToken {
        biscuit,
        serialized_biscuit,
        revocation_id,
        expired_at: Some(DateTime::from(expired_at)),
    })
}

const RESET_PASSWORD_TOKEN_VERSION: i64 = 1;
const RESET_PASSWORD_TOKEN_EXPIRATION: Duration = Duration::from_secs(60 * 30);

pub fn create_reset_password_token(
    private_key: &PrivateKey,
    user_id: Uuid,
) -> Result<RootToken, biscuit_auth::error::Token> {
    let keypair = KeyPair::from(private_key);
    let created_at = SystemTime::now();
    let expired_at = created_at + RESET_PASSWORD_TOKEN_EXPIRATION;

    let biscuit = biscuit!(
        r#"
            type("password_reset");
            version({RESET_PASSWORD_TOKEN_VERSION});
            user_id({user_id});
            created_at({created_at});
            expired_at({expired_at});
        "#,
    )
    .build(&keypair)?;
    let serialized_biscuit = biscuit.to_base64()?;
    let revocation_id = biscuit
        .revocation_identifiers()
        .first()
        .map(|rid| rid.to_owned())
        .ok_or(biscuit_auth::error::Token::InternalError)?;

    Ok(RootToken {
        biscuit,
        serialized_biscuit,
        revocation_id,
        expired_at: Some(DateTime::from(expired_at)),
    })
}

pub fn authorize_only_user(
    biscuit: &Biscuit,
    action: Action,
) -> Result<AuthorizedUserToken, biscuit_auth::error::Token> {
    match authorize(biscuit, action) {
        Ok(AuthorizedToken::User(aut)) => Ok(aut),
        #[allow(unreachable_patterns)]
        // This is unreachable because the only authorized token is a user token
        Ok(_) => {
            trace!("Authorization was denied because a user_access token was required");
            Err(biscuit_auth::error::Token::InternalError)
        }
        Err(e) => Err(e),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorizedRefreshToken {
    pub token_id: Uuid,
    pub session_id: Uuid,
    pub user_id: Uuid,
}

pub fn authorize_refresh_token(
    biscuit: &Biscuit,
) -> Result<AuthorizedRefreshToken, biscuit_auth::error::Token> {
    let mut authorizer = authorizer!(
        r#"
            supported_version("refresh", 1);
            valid_version($t, $v) <- type($t), version($v), supported_version($t, $v);
            check if valid_version($t, $v);

            expired($t) <- expired_at($exp), time($t), $exp < $t;
            deny if expired($t);
        "#
    );
    authorizer.set_time();
    authorizer.add_allow_all();

    authorizer.set_limits(AuthorizerLimits {
        max_time: Duration::from_millis(5),
        ..Default::default()
    });
    authorizer.add_token(biscuit)?;
    let result = authorizer.authorize();
    trace!("Authorizer state:\n{}", authorizer.print_world());
    result?;

    let raw_token_id: Vec<(Vec<u8>,)> = authorizer.query(rule!("data($id) <- token_id($id)"))?;
    let token_id = raw_token_id
        .first()
        .and_then(|(str,)| Uuid::from_slice(str).ok())
        .ok_or(biscuit_auth::error::Token::InternalError)?;
    let raw_session_id: Vec<(Vec<u8>,)> =
        authorizer.query(rule!("data($id) <- session_id($id)"))?;
    let session_id = raw_session_id
        .first()
        .and_then(|(str,)| Uuid::from_slice(str).ok())
        .ok_or(biscuit_auth::error::Token::InternalError)?;
    let raw_user_id: Vec<(Vec<u8>,)> = authorizer.query(rule!("data($id) <- user_id($id)"))?;
    let user_id = raw_user_id
        .first()
        .and_then(|(str,)| Uuid::from_slice(str).ok())
        .ok_or(biscuit_auth::error::Token::InternalError)?;

    Ok(AuthorizedRefreshToken {
        token_id,
        session_id,
        user_id,
    })
}

pub fn authorize(biscuit: &Biscuit, action: Action) -> Result<AuthorizedToken, error::Token> {
    let mut authorizer = authorizer!(
        r#"
            valid_types(["user_access", "service_access"]);
            valid_type($t) <- type($t), valid_types($vt), $vt.contains($t);
            check if valid_type($t);

            supported_version("user_access", 1);
            supported_version("service_access", 1);
            valid_version($t, $v) <- type($t), version($v), supported_version($t, $v);
            check if valid_version($t, $v);

            authorize_role($r) <- type("user_access"), role($r);
            valid_role($r) <- authorize_role($r), allowed_role($r);
            valid_role("service") <- type("service_access");
            check if valid_role($r);

            expired($t) <- expired_at($exp), time($t), $exp < $t;
            deny if expired($t);
        "#
    );

    authorizer.set_time();
    for fact in action.generate_facts() {
        authorizer.add_fact(fact)?;
    }
    authorizer.add_allow_all();

    authorizer.set_limits(AuthorizerLimits {
        max_time: Duration::from_millis(5),
        ..Default::default()
    });
    authorizer.add_token(biscuit)?;
    let result = authorizer.authorize();
    trace!("Authorizer state:\n{}", authorizer.print_world());
    result?;

    let raw_type: Vec<(String,)> = authorizer.query(rule!("data($id) <- type($id)"))?;
    let token_type = raw_type
        .first()
        .map(|(str,)| str)
        .ok_or(biscuit_auth::error::Token::InternalError)?;

    match token_type.as_str() {
        "user_access" => {
            let raw_session_id: Vec<(Vec<u8>,)> =
                authorizer.query(rule!("data($id) <- session_id($id)"))?;
            let session_id = raw_session_id
                .first()
                .and_then(|(str,)| Uuid::from_slice(str).ok())
                .ok_or(biscuit_auth::error::Token::InternalError)?;

            let raw_user_id: Vec<(Vec<u8>,)> =
                authorizer.query(rule!("data($id) <- user_id($id)"))?;
            let user_id = raw_user_id
                .first()
                .and_then(|(str,)| Uuid::from_slice(str).ok())
                .ok_or(biscuit_auth::error::Token::InternalError)?;

            let raw_email: Vec<(String,)> =
                authorizer.query(rule!("data($email) <- email($email)"))?;
            let email = raw_email
                .first()
                .ok_or(biscuit_auth::error::Token::InternalError)?
                .0
                .to_owned();

            let raw_username: Vec<(String,)> =
                authorizer.query(rule!("data($username) <- username($username)"))?;
            let username = raw_username
                .first()
                .ok_or(biscuit_auth::error::Token::InternalError)?
                .0
                .to_owned();

            let raw_role: Vec<(String,)> = authorizer.query(rule!("data($role) <- role($role)"))?;
            let role = raw_role
                .first()
                .ok_or(biscuit_auth::error::Token::InternalError)?
                .0
                .to_owned();

            Ok(AuthorizedToken::User(AuthorizedUserToken {
                session_id,
                user_id,
                email,
                username,
                role,
            }))
        }
        "service_access" => Ok(AuthorizedToken::Service),
        _ => {
            error!("Invalid token type: {}", token_type);
            Err(biscuit_auth::error::Token::InternalError)
        }
    }
}

pub fn authorize_email_verification(
    biscuit: &Biscuit,
) -> Result<AuthorizedEmailVerificationToken, biscuit_auth::error::Token> {
    let mut authorizer = authorizer!(
        r#"
            supported_version("email_verification", 1);
            valid_version($t, $v) <- type($t), version($v), supported_version($t, $v);
            check if valid_version($t, $v);

            expired($t) <- expired_at($exp), time($t), $exp < $t;
            deny if expired($t);
        "#
    );
    authorizer.set_time();
    authorizer.add_allow_all();

    authorizer.set_limits(AuthorizerLimits {
        max_time: Duration::from_secs(1800),
        ..Default::default()
    });
    authorizer.add_token(biscuit)?;
    let result = authorizer.authorize();
    trace!("Authorizer state:\n{}", authorizer.print_world());
    result?;

    let raw_user_id: Vec<(Vec<u8>,)> = authorizer.query(rule!("data($id) <- user_id($id)"))?;
    let user_id = raw_user_id
        .first()
        .and_then(|(str,)| Uuid::from_slice(str).ok())
        .ok_or(biscuit_auth::error::Token::InternalError)?;

    Ok(AuthorizedEmailVerificationToken { user_id })
}

pub fn get_user_id_from_expired_email_verification(
    biscuit: &Biscuit,
) -> Result<Uuid, biscuit_auth::error::Token> {
    let mut authorizer = authorizer!(
        r#"
            supported_version("email_verification", 1);
            valid_version($t, $v) <- type($t), version($v), supported_version($t, $v);
            check if valid_version($t, $v);
        "#
    );
    authorizer.add_allow_all();

    authorizer.set_limits(AuthorizerLimits {
        max_time: Duration::from_secs(1800),
        ..Default::default()
    });
    authorizer.add_token(biscuit)?;
    let result = authorizer.authorize();
    trace!("Authorizer state:\n{}", authorizer.print_world());
    result?;

    let raw_user_id: Vec<(Vec<u8>,)> = authorizer.query(rule!("data($id) <- user_id($id)"))?;
    let user_id = raw_user_id
        .first()
        .and_then(|(str,)| Uuid::from_slice(str).ok())
        .ok_or(biscuit_auth::error::Token::InternalError)?;

    Ok(user_id)
}

pub fn authorize_reset_password(
    biscuit: &Biscuit,
) -> Result<AuthorizedResetPasswordToken, biscuit_auth::error::Token> {
    let mut authorizer = authorizer!(
        r#"
            supported_version("password_reset", 1);
            valid_version($t, $v) <- type($t), version($v), supported_version($t, $v);
            check if valid_version($t, $v);

            expired($t) <- expired_at($exp), time($t), $exp < $t;
            deny if expired($t);
        "#
    );
    authorizer.set_time();
    authorizer.add_allow_all();

    authorizer.set_limits(AuthorizerLimits {
        max_time: Duration::from_secs(1800),
        ..Default::default()
    });
    authorizer.add_token(biscuit)?;
    let result = authorizer.authorize();
    trace!("Authorizer state:\n{}", authorizer.print_world());
    result?;

    let raw_user_id: Vec<(Vec<u8>,)> = authorizer.query(rule!("data($id) <- user_id($id)"))?;
    let user_id = raw_user_id
        .first()
        .and_then(|(str,)| Uuid::from_slice(str).ok())
        .ok_or(biscuit_auth::error::Token::InternalError)?;

    Ok(AuthorizedResetPasswordToken { user_id })
}

const REGISTRATION_TOKEN_VERSION: i64 = 1;
const REGISTRATION_TOKEN_EXPIRATION: Duration = Duration::from_secs(60 * 30);

pub fn create_registration_token(
    private_key: &PrivateKey,
) -> Result<RootToken, biscuit_auth::error::Token> {
    let keypair = KeyPair::from(private_key);
    let created_at = SystemTime::now();
    let expired_at = created_at + REGISTRATION_TOKEN_EXPIRATION;

    let biscuit = biscuit!(
        r#"
            type("registration");
            version({REGISTRATION_TOKEN_VERSION});
            created_at({created_at});
            expired_at({expired_at});
        "#,
    )
    .build(&keypair)?;
    let serialized_biscuit = biscuit.to_base64()?;
    let revocation_id = biscuit
        .revocation_identifiers()
        .first()
        .map(|rid| rid.to_owned())
        .ok_or(biscuit_auth::error::Token::InternalError)?;

    Ok(RootToken {
        biscuit,
        serialized_biscuit,
        revocation_id,
        expired_at: Some(DateTime::from(expired_at)),
    })
}

pub fn authorize_registration(biscuit: &Biscuit) -> Result<(), biscuit_auth::error::Token> {
    let mut authorizer = authorizer!(
        r#"
            supported_version("registration", 1);
            valid_version($t, $v) <- type($t), version($v), supported_version($t, $v);
            check if valid_version($t, $v);

            expired($t) <- expired_at($exp), time($t), $exp < $t;
            deny if expired($t);
        "#
    );
    authorizer.set_time();
    authorizer.add_allow_all();

    authorizer.set_limits(AuthorizerLimits {
        max_time: Duration::from_secs(1800),
        ..Default::default()
    });

    authorizer.add_token(biscuit)?;
    let result = authorizer.authorize();
    trace!("Authorizer state:\n{}", authorizer.print_world());
    result?;

    Ok(())
}

const SERVICE_ACCESS_TOKEN_VERSION: i64 = 1;

pub fn create_service_access_token(
    private_key: &PrivateKey,
    token_id: Uuid,
) -> Result<RootToken, biscuit_auth::error::Token> {
    let keypair = KeyPair::from(private_key);
    let created_at = SystemTime::now();

    let biscuit = biscuit!(
        r#"
            type("service_access");
            version({SERVICE_ACCESS_TOKEN_VERSION});
            token_id({token_id});
            created_at({created_at});
        "#,
    )
    .build(&keypair)?;
    let serialized_biscuit = biscuit.to_base64()?;
    let revocation_id = biscuit
        .revocation_identifiers()
        .first()
        .map(|rid| rid.to_owned())
        .ok_or(biscuit_auth::error::Token::InternalError)?;

    Ok(RootToken {
        biscuit,
        serialized_biscuit,
        revocation_id,
        expired_at: None,
    })
}
