//! PKCS#1 public key tests

use hex_literal::hex;
use pkcs1::RsaPublicKey;

/// RSA-2048 PKCS#1 public key encoded as ASN.1 DER.
///
/// Note: this key is extracted from the corresponding `rsa2048-priv.der`
/// example key in the `pkcs8` crate.
const RSA_2048_DER_EXAMPLE: &[u8] = include_bytes!("examples/rsa2048-pub.der");

/// RSA-4096 PKCS#1 public key encoded as ASN.1 DER
const RSA_4096_DER_EXAMPLE: &[u8] = include_bytes!("examples/rsa4096-pub.der");

// /// RSA-2048 PKCS#1 public key encoded as PEM
// #[cfg(feature = "pem")]
// const RSA_2048_PEM_EXAMPLE: &str = include_str!("examples/rsa2048-pub.pem");
//
// /// RSA-4096 PKCS#1 public key encoded as PEM
// #[cfg(feature = "pem")]
// const RSA_4096_PEM_EXAMPLE: &str = include_str!("examples/rsa4096-pub.pem");

#[test]
fn decode_rsa2048_der() {
    let key = RsaPublicKey::try_from(RSA_2048_DER_EXAMPLE).unwrap();

    // Extracted using:
    // $ openssl asn1parse -in tests/examples/rsa2048-pub.pem
    assert_eq!(
        key.modulus.as_bytes(),
        hex!(
            "B6C42C515F10A6AAF282C63EDBE24243A170F3FA2633BD4833637F47CA4F6F36E03A5D29EFC3191AC80F390D874B39E30F414FCEC1FCA0ED81E547EDC2CD382C76F61C9018973DB9FA537972A7C701F6B77E0982DFC15FC01927EE5E7CD94B4F599FF07013A7C8281BDF22DCBC9AD7CABB7C4311C982F58EDB7213AD4558B332266D743AED8192D1884CADB8B14739A8DADA66DC970806D9C7AC450CB13D0D7C575FB198534FC61BC41BC0F0574E0E0130C7BBBFBDFDC9F6A6E2E3E2AFF1CBEAC89BA57884528D55CFB08327A1E8C89F4E003CF2888E933241D9D695BCBBACDC90B44E3E095FA37058EA25B13F5E295CBEAC6DE838AB8C50AF61E298975B872F"
        )
    );
    assert_eq!(key.public_exponent.as_bytes(), hex!("010001"));
}

#[test]
fn decode_rsa4096_der() {
    let key = RsaPublicKey::try_from(RSA_4096_DER_EXAMPLE).unwrap();

    // Extracted using:
    // $ openssl asn1parse -in tests/examples/rsa4096-pub.pem
    assert_eq!(
        key.modulus.as_bytes(),
        hex!(
            "A7A74572811EA2617E49E85BD730DDE30F103F7D88EE3F765E540D3DD993BBB0BA140002859D0B40897436637F58B828EA74DF8321634077F99D4AA2D54CA375852EF597661D3713CE1EF3B4FD6A8E220238E467668A2C7EE3861D2212AE6A1EBDDFA88B62DF10F6BCF79EFF4AC298FB2563DF1B8764381AF9B1FB0CCD085E026B0AD9F6721A235177D0396B48754AD4A75242250A873BF2F6E7EE3C75DD613E365BA4F3210A6CC66B90A2FA3F762CA6884087B6BF8161EB144819F0F572F21F6C8E273E70D45A365B8B2819CE734613CC23B01329A17901F17078403861F54C52A051E2A58C75C2D9D80091BB9808A106C1F7ECB4034E15058BEEC725C5F919D62EAA234B62628D346C60BB919E70851DAB38571E6F0ED7634129F994EA368FEE7373DFDEC04445EBCA47FA20ED1540A860C948BABC98DA591CA1DE2E2E25540EF9B7CB353F60213B814A45D359EFA9B811EEFF08C65993BF8A85C2BFEAAA7ED5E6B43E18AE604464CE5F96150136E7D09F8B24FAD43D7870118CFA7BC24875506EBBC321B977E0861AEA50128620121F0B394A9CDD0A42411A1350C0770D975D71B00A90436240C967A0C3A5C20A0F6DE77F3F2CAFDA94ED0143C1F6E34F73E0CAC279EEEB7C637723A2B026C82802E1A4AEBAA8846DF98E7919498773E0D4F319956F4DE3AAD00EFB9A147D66B3AC1A01D35B2CFB48D400B0E7A80DC97551"
        )
    );
    assert_eq!(key.public_exponent.as_bytes(), hex!("010001"));
}

// TODO(tarcieri): test trait-based PEM decoding
// #[test]
// #[cfg(feature = "pem")]
// fn decode_rsa_2048_pem() {
//     let pkcs1_doc: Document = RSA_2048_PEM_EXAMPLE.parse().unwrap();
//     assert_eq!(pkcs1_doc.as_ref(), RSA_2048_DER_EXAMPLE);
//
//     // Ensure `Document` parses successfully
//     let pk = RsaPublicKey::try_from(RSA_2048_DER_EXAMPLE).unwrap();
//     assert_eq!(pkcs1_doc.decode().modulus.as_bytes(), pk.modulus.as_bytes());
// }
//
// #[test]
// #[cfg(feature = "pem")]
// fn decode_rsa_4096_pem() {
//     let pkcs1_doc: Document = RSA_4096_PEM_EXAMPLE.parse().unwrap();
//     assert_eq!(pkcs1_doc.as_ref(), RSA_4096_DER_EXAMPLE);
//
//     // Ensure `Document` parses successfully
//     let pk = RsaPublicKey::try_from(RSA_4096_DER_EXAMPLE).unwrap();
//     assert_eq!(pkcs1_doc.decode().modulus.as_bytes(), pk.modulus.as_bytes());
// }
