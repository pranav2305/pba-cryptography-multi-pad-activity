# Solution

The key is,
```
Bitcoin: A purely peer-to-peer version of electronic cash would allow online payments to be sent directly from one party to another without going through a financial institution.
```

## Steps

1. We XOR all the encrypted messages against each other.

This is because XOR have these set of properties,
* a xor 0 = a
* a xor b = b xor a
* a xor a = 0
* a xor (b xor c) = (a xor b) xor c

Therefore, when we XOR 2 of the cyphered messages against each other, the result is the XOR of the deciphered messages.
```
(a xor k ) xor (b xor k) = (a xor b) xor (k xor k)
                         = (a xor b) xor 0
                         = a xor b
```

2. Once we have all the possible XOR permutations (key XOR a, b, c ...), we can generate a new text from the highest frequency characters. We obtain part of the text from the key:

```
#itc###==#=p##ely=peer=to=pehr
```

3. From the above text we guess that the key is something along the lines of, **Bitcoin a purely peer to peer**. Next, we decrypt the other messages. We got the following messages,

```
the Tim:03/Jan/2009 Nhaccell
governm'Fs are good ay cxttin
bitcoin: A great as a kor` of
in ordehiFo have a dechntaliz
as soci=K becomes morh acd mo
i began:=] realize new-po~sibi
cryptoco;@encies allowhd con-c
not youhiYeys, Not you cbins.
bitcoin is purely peer to peer
```

4. Interestingly, this text looks very similar to the genesis block of bitcoin, written by Satoshi.
So, we try using the last sentence of the text as the key, which confirms our theory. The fully deciphered text reads:

```
The Times 03/Jan/2009 Chancellor on brink of second bailout for banks.
Governments are good at cutting off the heads of a centrally controlled networks like Napster, but pure P2P networks like Gnutella and Tor seem to be holding their own.
Bitcoin is great as a form of digital money, but its scripting language is too weak for any kind of serious advanced applications to be built on top.
In order to have a decentralized database, you need to have security. In order to have security, you need to have incentives.
As society becomes more and more complex, cheating will in many ways become progressively easier and easier to do and harder to police or even understand.
I began to realize new possibilities opening up between the fields of ICT and game theory, and the inevitable social change to which this would lead.
Cryptocurrencies allowed non-custodial exchange, without users having to sign up or create accounts.
Not your keys, Not your coins.
Bitcoin: A purely peer-to-peer version of electronic cash would allow online payments to be sent directly from one party to another without going through a financial institution.
```

## Answer

1. Find the length of the longest input cipher text.
   1. 178 characters. Which is the key
2. Generate a key of that length.
   1. Do we need to generate a key of that length?
3. Find what the correct key is...
   1. Bitcoin: A purely peer-to-peer version of electronic cash would allow online payments to be sent directly from one party to another without going through a financial institution.


```text
1f3cb1f3e01f3fd1f3ea1f3e61f3e01f3e71f3b31f3a91f3c81f3a91f3f91f3fc1f3fb1f3ec1f3e51f3f01f3a91f3f91f3ec1f3ec526e1b014a020411074c17111b1c071c4e4f0146430d0d08131d1d010707040017091648461e1d0618444f074c010e19594f0f1f1a07024e1d041719164e1c1652114f411645541b004e244f080213010c004c3b4c0911040e480e070b00310213101c4d0d4e00360b4f151a005253184913040e115454084f010f114554111d1a550f0d520401461f3e01f3e71f3e81f3e71f3ea1f3e01f3e81f3e51f3a91f3e01f3e71f3fa1f3fd1f3e01f3fd1f3fc1f3fd1f3e01f3e61f3e71f3a7
```

Notice a pattern? why might that be... 🤔

1f3 is use for padding to confuse the xor operation. After removing the below **1f3**, the output is,
```
Congratulations on championing the first of many assignments here at the Polkadot Blockchain Academy! We are so glad to have you here! 
```