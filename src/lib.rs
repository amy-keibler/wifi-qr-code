#![deny(missing_docs)]

//! Wifi QR codes are a way to encode wifi connection information and credentials into a QR code so that it can be scanned. They are supported via the latest Android and iOS phones, as well as other platforms.
//!
//! It is important to take into account that QR codes do not provide any security mechanisms that would prevent someone from just reading the code and recovering the password for the network. Android requires that you re-authenticate before it will display the QR code on the screen to make sure the user is allowed to share that information, for example.

pub use qrcode_generator::{QRCodeError, QrCodeEcc};

use std::io::Write;

/// Encode credentials as a matrix of boolean values. This is useful when manually generating an image.
///
/// # Examples
///
/// ```
/// use wifi_qr_code::QrCodeEcc;
/// use wifi_qr_code::{AuthenticationType, Visibility, WifiCredentials};
///
/// let wifi_credentials = WifiCredentials {
///     ssid: String::from("example ssid"),
///     authentication_type: AuthenticationType::WPA(String::from("example password")),
///     visibility: Visibility::Hidden,
/// };
/// wifi_qr_code::encode_as_matrix(&wifi_credentials, QrCodeEcc::Medium);
/// ```
pub fn encode_as_matrix(
    wifi_credentials: &WifiCredentials,
    qr_code_error_checking: QrCodeEcc,
) -> Result<Vec<Vec<bool>>, QRCodeError> {
    qrcode_generator::to_matrix(wifi_credentials.encode(), qr_code_error_checking)
}

/// Encode credentials as raw image data. This is useful when generating the QR code and then manipulating it with an image library.
///
/// # Examples
///
/// ```
/// use wifi_qr_code::QrCodeEcc;
/// use wifi_qr_code::{AuthenticationType, Visibility, WifiCredentials};
///
/// let wifi_credentials = WifiCredentials {
///     ssid: String::from("example ssid"),
///     authentication_type: AuthenticationType::WPA(String::from("example password")),
///     visibility: Visibility::Hidden,
/// };
/// wifi_qr_code::encode_as_image(&wifi_credentials, QrCodeEcc::Medium, 100);
/// ```
pub fn encode_as_image(
    wifi_credentials: &WifiCredentials,
    qr_code_error_checking: QrCodeEcc,
    image_size: usize,
) -> Result<Vec<u8>, QRCodeError> {
    qrcode_generator::to_image(
        wifi_credentials.encode(),
        qr_code_error_checking,
        image_size,
    )
}

/// Encode credentials as a PNG image.
///
/// # Examples
///
/// ```
/// use wifi_qr_code::QrCodeEcc;
/// use wifi_qr_code::{AuthenticationType, Visibility, WifiCredentials};
///
/// use std::fs::File;
///
/// let wifi_credentials = WifiCredentials {
///     ssid: String::from("example ssid"),
///     authentication_type: AuthenticationType::WPA(String::from("example password")),
///     visibility: Visibility::Hidden,
/// };
/// let png_file = File::create("wifi_qr.png").expect("Failed to create example PNG file.");
/// wifi_qr_code::encode_as_png(&wifi_credentials, QrCodeEcc::Medium, 100, png_file);
/// ```
pub fn encode_as_png(
    wifi_credentials: &WifiCredentials,
    qr_code_error_checking: QrCodeEcc,
    image_size: usize,
    mut writer: impl Write,
) -> Result<(), QRCodeError> {
    qrcode_generator::to_png_to_writer(
        wifi_credentials.encode(),
        qr_code_error_checking,
        image_size,
        &mut writer,
    )
}

/// Encode credentials as an SVG image.
///
/// # Examples
///
/// ```
/// use wifi_qr_code::QrCodeEcc;
/// use wifi_qr_code::{AuthenticationType, Visibility, WifiCredentials};
///
/// use std::fs::File;
///
/// let wifi_credentials = WifiCredentials {
///     ssid: String::from("example ssid"),
///     authentication_type: AuthenticationType::WPA(String::from("example password")),
///     visibility: Visibility::Hidden,
/// };
/// let svg_file = File::create("wifi_qr.svg").expect("Failed to create example SVG file.");
/// wifi_qr_code::encode_as_svg(&wifi_credentials, QrCodeEcc::Medium, 100, svg_file);
/// ```
pub fn encode_as_svg(
    wifi_credentials: &WifiCredentials,
    qr_code_error_checking: QrCodeEcc,
    image_size: usize,
    mut writer: impl Write,
) -> Result<(), QRCodeError> {
    qrcode_generator::to_png_to_writer(
        wifi_credentials.encode(),
        qr_code_error_checking,
        image_size,
        &mut writer,
    )
}

/// Declare whether the network is authenticated via WEP with a password, WPA with a password, or if the network is open.
pub enum AuthenticationType {
    /// WEP authentication is an older family of protocols. It is not particularly secure and wireless access points should use a more modern methods such as the WPA family of authentication protocols.
    WEP(String),
    /// WPA authentication is a more modern family of protocols. Typically, wireless networks will use WPA2 as their protocol implementation.
    WPA(String),
    /// No password / open access is particularly rare because it is possible for malicious actors to read all unencrypted traffic going across the network.
    NoPassword,
}

impl AuthenticationType {
    fn encode(&self) -> String {
        match self {
            Self::WEP(password) => format!("T:WEP;P:{};", escape(password)),
            Self::WPA(password) => format!("T:WPA;P:{};", escape(password)),
            Self::NoPassword => String::from("T:nopass;"),
        }
    }
}

/// Declare whether the network is broadcasting its availability.
pub enum Visibility {
    /// Visible wifi networks display in lists of networks when a device scans an area.
    Visible,
    /// Hidden wifi networks do not show up on scans and must be known by their SSID to be accessed.
    Hidden,
}

impl Visibility {
    fn encode(&self) -> String {
        match self {
            Self::Visible => String::from("H:false;"),
            Self::Hidden => String::from("H:true;"),
        }
    }
}

/// The credentials needed to completely connect to a wifi network.
pub struct WifiCredentials {
    /// The SSID of a wifi network is the name used to access it.
    pub ssid: String,
    /// The authentication type of a wifi network determines the protocol used to access it and the password required to properly authenticate to it.
    pub authentication_type: AuthenticationType,
    /// The visibility of a wifi network determines if it can be seen by any device or if it must be known by SSID beforehand.
    pub visibility: Visibility,
}

impl WifiCredentials {
    /// Encode the credentials into the form expected for a wifi QR Code. Special characters (i.e. ";,:\) will be escaped in the output.
    ///
    /// # Examples
    ///
    /// ```
    /// use wifi_qr_code::{AuthenticationType, Visibility, WifiCredentials};
    ///
    /// let wifi_credentials = WifiCredentials {
    ///     ssid: String::from("example ssid"),
    ///     authentication_type: AuthenticationType::WPA(String::from("example password")),
    ///     visibility: Visibility::Hidden,
    /// };
    /// assert_eq!("WIFI:S:example ssid;T:WPA;P:example password;H:true;;", wifi_credentials.encode());
    /// ```
    pub fn encode(&self) -> String {
        format!(
            "WIFI:{}{}{};",
            self.encode_ssid(),
            self.authentication_type.encode(),
            self.visibility.encode()
        )
    }

    fn encode_ssid(&self) -> String {
        format!("S:{};", escape(&self.ssid))
    }
}

fn escape(input: &str) -> String {
    String::from(input)
        .replace(r#"\"#, r#"\\"#)
        .replace(r#"""#, r#"\""#)
        .replace(";", r#"\;"#)
        .replace(",", r#"\,"#)
        .replace(":", r#"\:"#)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_encodes_valid_wifi_forms() {
        // WIFI:S:<SSID>;T:<WPA|WEP|>;P:<password>;H:<true|false|>;
        let wifi_credentials = WifiCredentials {
            ssid: String::from("test ssid"),
            authentication_type: AuthenticationType::WEP(String::from("test password")),
            visibility: Visibility::Visible,
        };
        assert_eq!(
            "WIFI:S:test ssid;T:WEP;P:test password;H:false;;",
            &wifi_credentials.encode()
        );
        let wifi_credentials = WifiCredentials {
            ssid: String::from("test ssid"),
            authentication_type: AuthenticationType::WPA(String::from("test password")),
            visibility: Visibility::Hidden,
        };
        assert_eq!(
            "WIFI:S:test ssid;T:WPA;P:test password;H:true;;",
            &wifi_credentials.encode()
        );
        let wifi_credentials = WifiCredentials {
            ssid: String::from("test ssid"),
            authentication_type: AuthenticationType::NoPassword,
            visibility: Visibility::Visible,
        };
        assert_eq!(
            "WIFI:S:test ssid;T:nopass;H:false;;",
            &wifi_credentials.encode()
        );
    }

    #[test]
    fn it_properly_handles_escaped_characters() {
        let wifi_credentials = WifiCredentials {
            ssid: String::from(r#"special_characters ";,:\"#),
            authentication_type: AuthenticationType::WEP(String::from(
                r#"special_characters ";,:\"#,
            )),
            visibility: Visibility::Visible,
        };
        assert_eq!(
            r#"WIFI:S:special_characters \"\;\,\:\\;T:WEP;P:special_characters \"\;\,\:\\;H:false;;"#,
            &wifi_credentials.encode()
        );
    }
}
