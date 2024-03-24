use lambdaworks_math::elliptic_curve::short_weierstrass::curves::bls12_381::curve::BLS12381Curve;
use lambdaworks_math::elliptic_curve::short_weierstrass::point::ShortWeierstrassProjectivePoint;
use lambdaworks_math::cyclic_group::IsGroup;
use lambdaworks_math::elliptic_curve::traits::IsEllipticCurve;
use lambdaworks_math::unsigned_integer::element::U256;
use lambdaworks_math::unsigned_integer::traits::IsUnsignedInteger;
use lambdaworks_math::traits::{AsBytes, ByteConversion};



pub fn compute_publickey<T: IsUnsignedInteger>(
    private_key: T,
) -> ShortWeierstrassProjectivePoint<BLS12381Curve> {
    BLS12381Curve::generator().operate_with_self(private_key)
}

fn main() {
    let private_key = U256::from_hex_unchecked("6C616D6264617370");
    let public_key = compute_publickey(private_key);

    let public_key_bytes = public_key.as_bytes();

    let public_u256 =
        U256::from_bytes_be(&public_key_bytes).expect("Public key conversion to U256 Failed!");

    println!("Public key calculated from  {:?} = {:?}",private_key.to_hex() , public_u256.to_hex());
}

