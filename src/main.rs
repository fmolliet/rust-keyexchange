pub mod utils;

use openssl::dh::Dh;
use openssl::error::ErrorStack;
//use openssl::ssl::{SslContext, SslMethod};

use crate::utils::encode_hex;

fn main() -> Result<(), ErrorStack> {
   // Verificar se o fips 140-2 está habilitado
    println!("FIPS-140-2 Enabled: {:?}", openssl::fips::enabled());

    // Inicialize o contexto SSL com o método de criptografia TLSv1.2
    //let ctx = SslContext::builder(SslMethod::tls())?.build();

    // Crie o objeto DH com os parâmetros Diffie-Hellman padrão
    let dh = Dh::get_1024_160()?;

    // Gere as chaves privadas e públicas
    let bob = dh.generate_key()?;

    // Imprima a chave pública
    println!("Chave pública gerada: {:?}", bob.public_key());
    println!("Chave privada gerada: {:?}", bob.private_key());
    println!("***********************");

    // Gere as chaves privadas e públicas
    let dh = Dh::get_1024_160()?;
    let alice = dh.generate_key()?;

    println!("Chave pública gerada: {:?}", alice.public_key());
    println!("Chave privada gerada: {:?}", alice.private_key());
    println!("***********************");

    // Computa as chaves para alice e bob
    let alice_secret = alice.compute_key(bob.public_key())?;
    let bob_secret = bob.compute_key(alice.public_key())?;
    println!("Tamanho da chave = {}", alice_secret.len());
    println!("***********************");
    println!("Secret da Alice calculado: {:?}", alice_secret);
    println!("Secret do Bob calculado: {:?}", bob_secret);

    println!("***********************");
    println!(
        "Secret Hex da Alice calculado: {:?}",
        encode_hex(&utils::vec_to_array::<u8, 128>(alice_secret))
    );
    println!(
        "Secret Hex da Bob calculado: {:?}",
        encode_hex(&utils::vec_to_array::<u8, 128>(bob_secret))
    );

    Ok(())
}
