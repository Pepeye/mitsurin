/*
 * ORY Hydra
 *
 * Welcome to the ORY Hydra HTTP API documentation. You will find documentation for all HTTP APIs here.
 *
 * The version of the OpenAPI document: latest
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct JsonWebKeySetGeneratorRequest {
    /// The algorithm to be used for creating the key. Supports \"RS256\", \"ES512\", \"HS512\", and \"HS256\"
    #[serde(rename = "alg")]
    pub alg: String,
    /// The kid of the key to be created
    #[serde(rename = "kid")]
    pub kid: String,
    /// The \"use\" (public key use) parameter identifies the intended use of the public key. The \"use\" parameter is employed to indicate whether a public key is used for encrypting data or verifying the signature on data. Valid values are \"enc\" and \"sig\".
    #[serde(rename = "use")]
    pub _use: String,
}

impl JsonWebKeySetGeneratorRequest {
    pub fn new(alg: String, kid: String, _use: String) -> JsonWebKeySetGeneratorRequest {
        JsonWebKeySetGeneratorRequest {
            alg,
            kid,
            _use,
        }
    }
}


