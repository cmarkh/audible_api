use crate::{
    localization::{self},
    Result,
};

use super::{
    oauth::{build_oauth_url, extract_auth_code},
    register::register,
    Auth,
};

/// Take oauth_url and return the openid.oa2.authorization_code
pub type GetAuthorizationCode = fn(String) -> Result<String>;

pub async fn sign_in(
    country_code: &str,
    device_serial: Option<String>,
    with_username: bool,
    get_authorization_code: Option<GetAuthorizationCode>,
) -> Result<Auth> {
    let locale =
        localization::find_by_country_code(country_code).ok_or("Country code not found")?;

    let (oauth_url, code_verifier, device_serial) = build_oauth_url(
        &locale.country_code,
        &locale.domain,
        &locale.market_place_id,
        device_serial,
        false,
    )?;

    let auth_code = match get_authorization_code {
        Some(f) => f(oauth_url)?,
        None => open_browser_for_auth_code(&oauth_url)?,
    };

    let registration = register(
        &auth_code,
        &code_verifier,
        &locale.domain,
        &device_serial,
        with_username,
    )
    .await?;

    Ok(Auth {
        locale,
        device_registration: registration,
        authorization_code: auth_code,
        code_verifier,
    })
}

pub fn open_browser_for_auth_code(url: &str) -> Result<String> {
    // Opens the URL in the default web browser
    let response_url = match webbrowser::open(url) {
        Ok(_) => {
            println!("Opened {} in your default web browser.", url);
            println!("Please log in and copy the resulting URL from your browser's address bar:");

            // URL is too long for the standard buffer
            let term = console::Term::stdout();
            let mut input = String::new();

            loop {
                if let Ok(char) = term.read_char() {
                    match char {
                        '\n' => {
                            break;
                        }
                        c => {
                            if c == '&' || c == '?' {
                                println!();
                            }
                            print!("{c}");
                            input.push(c)
                        }
                    }
                }
            }
            println!();

            Ok(input.trim().to_string())
        }
        Err(e) => Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to open URL: {}", e),
        )),
    }?;

    extract_auth_code(&response_url)
}
