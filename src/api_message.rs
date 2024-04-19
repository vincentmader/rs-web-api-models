use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ApiMessage {
    Ok(ApiOk),
    Err(ApiError),
}

impl From<ApiMessage> for String {
    fn from(msg: ApiMessage) -> Self {
        match msg {
            ApiMessage::Ok(msg) => match msg {
                ApiOk::LoginSucceeded => "Login succeeded.",
                ApiOk::RegistrationSucceeded => "Registration succeeded.",
                ApiOk::PasswordResetMailWasSent => "Password reset mail was sent.",
                ApiOk::FileUploadSucceeded => "File upload succeeded.",
            },
            ApiMessage::Err(msg) => match msg {
                ApiError::LoginError(msg) => match msg {
                    LoginError::InvalidLoginCredentials => "Invalid login credentials.",
                    LoginError::EmptyUserInfo => "Please enter your mail address or user name.",
                    LoginError::EmptyPassWord => "Please enter your password.",
                },
                ApiError::RegistrationError(msg) => match msg {
                    RegistrationError::EmptyUserName => "Please enter your user name.",
                    RegistrationError::EmptyPassWord => "Please enter your password.",
                    RegistrationError::EmptyMailAddress => "Please enter your mail address.",
                    RegistrationError::EmptyPassWordConfirm => "Please confirm your password.",
                    RegistrationError::UserNameExistsAlready => "Account with this user name exists already.",
                    RegistrationError::InvalidUserNameFormat => "Invalid user name format: It should contain only alphanumeric characters, and have a length of at least 3.",
                    RegistrationError::InvalidPasswordFormat => "Invalid password format: It should contain at least one lower-case character, one upper-case character, one number, one special character, and have a length of at least 8.",
                    RegistrationError::MailAddressExistsAlready => "Account with this mail address exists already.",
                    RegistrationError::InvalidMailAddressFormat => "Invalid mail address format.",
                    RegistrationError::InvalidPasswordConfirmation => "Password confirmation does not match password.",
                },
                ApiError::PasswordResetError(msg) => match msg {
                    PasswordResetError::AccountDoesNotExist => "Account does not exist.",
                    PasswordResetError::EmptyUserInfo => {
                        "Please enter your mail address or user name."
                    }
                },
                ApiError::DatabaseError(msg) => match msg {
                    DatabaseError::CouldNotSaveToDatabase => "Internal server error: Could not save to database. There is nothing you can do about this, please contact your admin!",
                },
            },
        }
        .to_owned()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ApiOk {
    LoginSucceeded,
    RegistrationSucceeded,
    PasswordResetMailWasSent,
    FileUploadSucceeded,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ApiError {
    LoginError(LoginError),
    RegistrationError(RegistrationError),
    PasswordResetError(PasswordResetError),
    DatabaseError(DatabaseError),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum LoginError {
    EmptyUserInfo,
    EmptyPassWord,
    InvalidLoginCredentials,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum RegistrationError {
    EmptyMailAddress,
    EmptyUserName,
    EmptyPassWord,
    EmptyPassWordConfirm,
    UserNameExistsAlready,
    MailAddressExistsAlready,
    InvalidUserNameFormat,
    InvalidMailAddressFormat,
    InvalidPasswordFormat,
    InvalidPasswordConfirmation,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PasswordResetError {
    AccountDoesNotExist,
    EmptyUserInfo,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum DatabaseError {
    CouldNotSaveToDatabase,
}
