use rustls::client::{
    HandshakeSignatureValid, ServerCertVerified, ServerCertVerifier, WebPkiVerifier,
};
use rustls::internal::msgs::enums::AlertDescription;
use rustls::internal::msgs::handshake::DigitallySignedStruct;
use rustls::{Certificate, Error, SignatureScheme};
use std::sync::Arc;


pub struct MockServerVerifier {
    cert_rejection_error: Option<Error>,
    tls12_signature_error: Option<Error>,
    tls13_signature_error: Option<Error>,
    wants_scts: bool,
    signature_schemes: Vec<SignatureScheme>,
}

impl ServerCertVerifier for MockServerVerifier {
    fn verify_server_cert(
        &self,
        end_entity: &rustls::Certificate,
        intermediates: &[rustls::Certificate],
        server_name: &rustls::ServerName,
        scts: &mut dyn Iterator<Item = &[u8]>,
        oscp_response: &[u8],
        now: std::time::SystemTime,
    ) -> Result<ServerCertVerified, Error> {
        let scts: Vec<Vec<u8>> = scts.map(|x| x.to_owned()).collect();
        // println!(
        //     "verify_server_cert({:?}, {:?}, {:?}, {:?}, {:?}, {:?})",
        //     end_entity, intermediates, server_name, scts, oscp_response, now
        // );
        if let Some(error) = &self.cert_rejection_error {
            Err(error.clone())
        } else {
            Ok(ServerCertVerified::assertion())
        }
    }

    fn verify_tls12_signature(
        &self,
        message: &[u8],
        cert: &Certificate,
        dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, Error> {
        // println!(
        //     "verify_tls12_signature({:?}, {:?}, {:?})",
        //     message, cert, dss
        // );
        if let Some(error) = &self.tls12_signature_error {
            Err(error.clone())
        } else {
            Ok(HandshakeSignatureValid::assertion())
        }
    }

    fn verify_tls13_signature(
        &self,
        message: &[u8],
        cert: &Certificate,
        dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, Error> {
        // println!(
        //     "verify_tls13_signature({:?}, {:?}, {:?})",
        //     message, cert, dss
        // );
        if let Some(error) = &self.tls13_signature_error {
            Err(error.clone())
        } else {
            Ok(HandshakeSignatureValid::assertion())
        }
    }

    fn supported_verify_schemes(&self) -> Vec<SignatureScheme> {
        self.signature_schemes.clone()
    }

    fn request_scts(&self) -> bool {
        // println!("request_scts? {:?}", self.wants_scts);
        self.wants_scts
    }
}

impl MockServerVerifier {
    pub fn accepts_anything() -> Self {
        MockServerVerifier {
            cert_rejection_error: None,
            ..Default::default()
        }
    }

    pub fn rejects_certificate(err: Error) -> Self {
        MockServerVerifier {
            cert_rejection_error: Some(err),
            ..Default::default()
        }
    }

    pub fn rejects_tls12_signatures(err: Error) -> Self {
        MockServerVerifier {
            tls12_signature_error: Some(err),
            ..Default::default()
        }
    }

    pub fn rejects_tls13_signatures(err: Error) -> Self {
        MockServerVerifier {
            tls13_signature_error: Some(err),
            ..Default::default()
        }
    }

    pub fn offers_no_signature_schemes() -> Self {
        MockServerVerifier {
            signature_schemes: vec![],
            ..Default::default()
        }
    }
}

impl Default for MockServerVerifier {
    fn default() -> Self {
        MockServerVerifier {
            cert_rejection_error: None,
            tls12_signature_error: None,
            tls13_signature_error: None,
            wants_scts: false,
            signature_schemes: WebPkiVerifier::verification_schemes(),
        }
    }
}