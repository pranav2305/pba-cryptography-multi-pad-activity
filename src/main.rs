use array_bytes::{bytes2hex, hex2bytes_unchecked};

fn xor_bytes(a: &[u8], b: &[u8]) -> Vec<u8> {
    a.iter().zip(b.iter()).map(|(x, y)| x ^ y).collect()
}

pub fn encrypt(key: &[u8], msg: &str) -> String {
    let c = xor_bytes(key, msg.as_bytes());
    bytes2hex("", &c)
}

pub fn decrypt(key: &[u8], cipher: &str) -> String {
    let cipher = hex2bytes_unchecked(cipher);
    String::from_utf8_lossy(&xor_bytes(key, &cipher)).to_string()
}

fn main() {
    let key = "Bitcoin: A purely peer-to-peer version of electronic cash would allow online payments to be sent directly from one party to another without going through a financial institution.";
    let ciphertexts = vec![
        "160111433b00035f536110435a380402561240555c526e1c0e431300091e4f04451d1d490d1c49010d000a0a4510111100000d434202081f0755034f13031600030d0204040e",
        "050602061d07035f4e3553501400004c1e4f1f01451359540c5804110c1c47560a1415491b06454f0e45040816431b144f0f4900450d1501094c1b16550f0b4e151e03031b450b4e020c1a124f020a0a4d09071f16003a0e5011114501494e16551049021011114c291236520108541801174b03411e1d124554284e141a0a1804045241190d543c00075453020a044e134f540a174f1d080444084e01491a090b0a1b4103570740",
        "000000000000001a49320017071704185941034504524b1b1d40500a0352441f021b0708034e4d0008451c40450101064f071d1000100201015003061b0b444c00020b1a16470a4e051a4e114f1f410e08040554154f064f410c1c00180c0010000b0f5216060605165515520e09560e00064514411304094c1d0c411507001a1b45064f570b11480d001d4c134f060047541b185c",
        "0b07540c1d0d0b4800354f501d131309594150010011481a1b5f11090c0845124516121d0e0c411c030c45150a16541c0a0b0d43540c411b0956124f0609075513051816590026004c061c014502410d024506150545541c450110521a111758001d0607450d11091d00121d4f0541190b45491e02171a0d49020a534f",
        "031a5410000a075f5438001210110a011c5350080a0048540e431445081d521345111c041f0245174a0006040002001b01094914490f0d53014e570214021d00160d151c57420a0d03040b4550020e1e1f001d071a56110359420041000c0b06000507164506151f104514521b02000b0145411e05521c1852100a52411a0054180a1e49140c54071d5511560201491b0944111a011b14090c0e41",
        "0b4916060808001a542e0002101309050345500b00050d04005e030c071b4c1f111b161a4f01500a08490b0b451604520d0b1d1445060f531c48124f1305014c051f4c001100262d38490f0b4450061800004e001b451b1d594e45411d014e004801491b0b0602050d41041e0a4d53000d0c411c41111c184e130a0015014f03000c1148571d1c011c55034f12030d4e0b45150c5c",
        "011b0d131b060d4f5233451e161b001f59411c090a0548104f431f0b48115505111d17000e02000a1e430d0d0b04115e4f190017480c14074855040a071f4448001a050110001b014c1a07024e5014094d0a1c541052110e54074541100601014e101a5c",
        "0c06004316061b48002a4509065e45221654501c0a075f540c42190b165c",
        "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
    ];

    let cipher: Vec<_> = ciphertexts.iter().map(hex2bytes_unchecked).collect();

    for j in 1..ciphertexts.len() {
        let mut result = Vec::new();
        for i in 1..ciphertexts.len() {
            if i == j {
                continue;
            }
            let raw_xor = xor_bytes(&cipher[j], &cipher[i]);
            let filtered_raw: Vec<_> = raw_xor
                .into_iter()
                .map(|ch| if ch.is_ascii_alphabetic() { ch } else { b'_' })
                .collect();
            result.push(filtered_raw.clone());
            println!("{}", String::from_utf8(filtered_raw).unwrap());
        }

        let min_length = cipher.iter().map(|x| x.len()).min().unwrap();
        let mut plain_text = vec![b'='; min_length];

        for i in 0..min_length {
            let mut count_ = 0;
            let mut count_l = 0;
            let mut curr_letter = 0u8;
            let mut only = true;
            result
                .iter()
                .filter(|bytes| bytes.len() > i)
                .map(|bytes| bytes[i])
                .for_each(|letter| {
                    if letter == b'_' {
                        count_ += 1;
                    } else {
                        count_l += 1;
                        if curr_letter != 0 && curr_letter == letter.to_ascii_lowercase() {
                            // do nothing
                        } else if curr_letter != 0 && curr_letter != letter.to_ascii_lowercase() {
                            only = false;
                        }
                        curr_letter = letter.to_ascii_lowercase();
                    }
                });
            if count_l == 1 {
                plain_text[i] = curr_letter;
            } else if count_l > 1 {
                if only {
                    plain_text[i] = curr_letter;
                }
            } else {
                plain_text[i] = b'#';
            }
        }
        println!();
        println!("{}", String::from_utf8(plain_text).unwrap());
    }

    for ciphertext in ciphertexts {
        let message = decrypt(key.as_bytes(), ciphertext);
        println!("{}", message);
    }

    let last_ciphertext = decrypt(key.as_bytes(), "cbe0fdeae6e0e7b3a9c8a9f9fcfbece5f0a9f9ecec526e1b014a020411074c17111b1c071c4e4f0146430d0d08131d1d010707040017091648461e1d0618444f074c010e19594f0f1f1a07024e1d041719164e1c1652114f411645541b004e244f080213010c004c3b4c0911040e480e070b00310213101c4d0d4e00360b4f151a005253184913040e115454084f010f114554111d1a550f0d52040146e0e7e8e7eae0e8e5a9e0e7fafde0fdfcfde0e6e7a7");
    println!("{}", last_ciphertext);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encrypt_should_work() {
        let plaintext = "The Times 03/Jan/2009 Chancellor on brink of second bailout for banks.";
        let key = "Bitcoin: A purely peer-to-peer version of electronic cash would allow online payments to be sent directly from one party to another without going through a financial institution.";
        let cipher = "160111433b00035f536110435a380402561240555c526e1c0e431300091e4f04451d1d490d1c49010d000a0a4510111100000d434202081f0755034f13031600030d0204040e";

        assert_eq!(encrypt(key.as_bytes(), plaintext), cipher);
    }
}
