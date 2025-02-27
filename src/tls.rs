use anyhow::Result;
use rcgen::{Certificate, CertificateParams, DistinguishedName, DnType, PKCS_ECDSA_P256_SHA256};
use rustls::pki_types::CertificateDer;
use rustls::ServerConfig;
use std::{fs, path::Path, sync::Arc};

pub fn create_self_signed_cert(cert_path: &Path, key_path: &Path) -> Result<()> {
    let mut params = CertificateParams::new(vec!["localhost".to_string()]);
    params.distinguished_name = DistinguishedName::new();
    params
        .distinguished_name
        .push(DnType::CommonName, "localhost");
    params.alg = &PKCS_ECDSA_P256_SHA256;

    let cert = Certificate::from_params(params)?;

    fs::write(cert_path, cert.serialize_pem()?)?;
    fs::write(key_path, cert.serialize_private_key_pem())?;

    Ok(())
}

pub fn load_tls_config(cert_path: &Path, key_path: &Path) -> Result<Arc<ServerConfig>> {
    // Load certificate and private key
    let cert_pem = fs::read(cert_path)?;
    let key_pem = fs::read(key_path)?;

    // Parse certificate
    let certs = rustls_pemfile::certs(&mut cert_pem.as_slice())
        .collect::<Result<Vec<CertificateDer<'static>>, _>>()?;

    // Parse private key
    let key = rustls_pemfile::private_key(&mut key_pem.as_slice())?.unwrap();

    // Create TLS config
    let config = ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(certs, key)?;

    Ok(Arc::new(config))
}
