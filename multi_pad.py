import codecs

key = "Bitcoin: A purely peer-to-peer version of electronic cash would allow online payments to be sent directly from one party to another without going through a financial institution."
cyphertexts = [
    "160111433b00035f536110435a380402561240555c526e1c0e431300091e4f04451d1d490d1c49010d000a0a4510111100000d434202081f0755034f13031600030d0204040e",
    "050602061d07035f4e3553501400004c1e4f1f01451359540c5804110c1c47560a1415491b06454f0e45040816431b144f0f4900450d1501094c1b16550f0b4e151e03031b450b4e020c1a124f020a0a4d09071f16003a0e5011114501494e16551049021011114c291236520108541801174b03411e1d124554284e141a0a1804045241190d543c00075453020a044e134f540a174f1d080444084e01491a090b0a1b4103570740",
    "000000000000001a49320017071704185941034504524b1b1d40500a0352441f021b0708034e4d0008451c40450101064f071d1000100201015003061b0b444c00020b1a16470a4e051a4e114f1f410e08040554154f064f410c1c00180c0010000b0f5216060605165515520e09560e00064514411304094c1d0c411507001a1b45064f570b11480d001d4c134f060047541b185c",
    "0b07540c1d0d0b4800354f501d131309594150010011481a1b5f11090c0845124516121d0e0c411c030c45150a16541c0a0b0d43540c411b0956124f0609075513051816590026004c061c014502410d024506150545541c450110521a111758001d0607450d11091d00121d4f0541190b45491e02171a0d49020a534f",
    "031a5410000a075f5438001210110a011c5350080a0048540e431445081d521345111c041f0245174a0006040002001b01094914490f0d53014e570214021d00160d151c57420a0d03040b4550020e1e1f001d071a56110359420041000c0b06000507164506151f104514521b02000b0145411e05521c1852100a52411a0054180a1e49140c54071d5511560201491b0944111a011b14090c0e41",
    "0b4916060808001a542e0002101309050345500b00050d04005e030c071b4c1f111b161a4f01500a08490b0b451604520d0b1d1445060f531c48124f1305014c051f4c001100262d38490f0b4450061800004e001b451b1d594e45411d014e004801491b0b0602050d41041e0a4d53000d0c411c41111c184e130a0015014f03000c1148571d1c011c55034f12030d4e0b45150c5c",
    "011b0d131b060d4f5233451e161b001f59411c090a0548104f431f0b48115505111d17000e02000a1e430d0d0b04115e4f190017480c14074855040a071f4448001a050110001b014c1a07024e5014094d0a1c541052110e54074541100601014e101a5c",
    "0c06004316061b48002a4509065e45221654501c0a075f540c42190b165c",
    "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
]

def main():
    cipher = [hex_to_str(cyphertext) for cyphertext in cyphertexts]

    for j in range(1, len(cyphertexts)):
        result = []
        for i in range(1, len(cyphertexts)):
            if i == j:
                continue
            raw_xor = xor_strings(cipher[j], cipher[i])
            filtered_raw = [
                char if char.isalpha() else "_" for char in raw_xor
            ]
            result.append("".join(filtered_raw))
            print("".join(filtered_raw))

        min_length = min(len(cipher[j]), *map(len, cipher))
        plain_text = ["="] * min_length

        for i in range(min_length):
            count_ = 0
            countL = 0
            curr_letter = ""
            only = True
            for letter in [string[i] for string in result if len(string) > i]:
                if letter == "_":
                    count_ += 1
                else:
                    countL += 1
                    if curr_letter != "" and curr_letter == letter.lower():
                        pass
                    elif curr_letter != "" and curr_letter != letter.lower():
                        only = False
                    curr_letter = letter.lower()

            if countL == 1:
                plain_text[i] = curr_letter
            elif countL > 1:
                if only:
                    plain_text[i] = curr_letter
            else:
                plain_text[i] = "#"
        print()
        print("".join(plain_text))

def hex_to_str(hex_string):
    return "".join([chr(int(hex_string[i:i+2], 16)) for i in range(0, len(hex_string), 2)])

def int_str(h):
    return [int(h[2 * i : 2 * i + 2], 16) for i in range(len(h) // 2)]

def xor_strings(str1, str2):
    return "".join(chr(ord(a) ^ ord(b)) for a, b in zip(str1, str2))

def encrypt(key, msg):
    c = xor_strings(key, msg)
    return codecs.encode(bytes(c, encoding="utf-8"), "hex")

def decrypt(key, cipher):
    cipher = "".join([chr(x) for x in int_str(cipher)])
    return xor_strings(key, cipher)

if __name__ == "__main__":
    main()
    for cyphertext in cyphertexts:
        message = decrypt(key, cyphertext)
        print(message)

    last_ciphertext = decrypt(key,"cbe0fdeae6e0e7b3a9c8a9f9fcfbece5f0a9f9ecec526e1b014a020411074c17111b1c071c4e4f0146430d0d08131d1d010707040017091648461e1d0618444f074c010e19594f0f1f1a07024e1d041719164e1c1652114f411645541b004e244f080213010c004c3b4c0911040e480e070b00310213101c4d0d4e00360b4f151a005253184913040e115454084f010f114554111d1a550f0d52040146e0e7e8e7eae0e8e5a9e0e7fafde0fdfcfde0e6e7a7")
    print(last_ciphertext)