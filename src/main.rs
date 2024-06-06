use serde::{Deserialize, Serialize};

use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    exp: u64,
}

fn main() {
    
    // ================== ENCODING ======================
    let my_claims: Claims =
        Claims { sub: "token_xyz/table_list".to_owned(), company: "ACME".to_owned(), exp: 10000000000 };
    let key = b"secret";

    let header =
        Header { kid: Some("signing_key".to_owned()), alg: Algorithm::HS512, ..Default::default() };

    // match control flow
    let token = match encode(&header, &my_claims, &EncodingKey::from_secret(key)) {
        Ok(t) => t,
        Err(_) => panic!(), // in practice you would return the error
    };
    println!("{:?}", token);
    //=========================================== 

    // DECODING
    let token_data: jsonwebtoken::TokenData<Claims> = match decode::<Claims>(
        &token,
        &DecodingKey::from_secret(key),
        &Validation::new(Algorithm::HS512),
    ) {
        Ok(c) => c,
        Err(err) => match *err.kind() {
            ErrorKind::InvalidToken => panic!(), // Example on how to handle a specific error
            _ => panic!(),
        },
    };

    println!("\n\n\n");
    println!("{:?}", token_data.claims);
    println!("{:?}", token_data.header);

    println!("\n\n\n");
}
