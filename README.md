# Solution

The key is,
```
Bitcoin: A purely peer-to-peer version of electronic cash would allow online payments to be sent directly from one party to another without going through a financial institution.
```

## Steps

1. We XOR all the encrypted messages.

This is because XOR have these set of preoperties,
* a xor 0 = a
* a xor b = b xor a
* a xor a = 0
* a xor (b xor c) = (a xor b) xor c

When we XOR both the messages we have a xor b,
```
(a xor k ) xor (b xor k) = (a xor b) xor (k xor k) 
                         = (a xor b) xor 0 
                         = a xor b
```

2. After all the XOR operations (key XOR a, b, c ...), we can generate a new text from the highest frequency characters. We obtain part of the text from the key,

```
#itc###==#=p##ely=peer=to=pehr
```

3. From the above text we guess that the key is something along the lines of, **bitcoin a purely peer to peer**. Next, we decrypt the other messages. We got the following messages,

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

4. Interestingly, the messages look like it is in the genesis block of bitcoin written by Satoshi. Next, we decrypt the key from the first encrypted messages from his actual text. We got,
```
The Times 03/Jan/2009 Chancellor on brink of second bailout for banks
Governments are good at cutting off the heads of a centrally controll
Bitcoin is great as a form of digital money, but its scripting langua
In order to have a decentralized database, you need to have security.
As society becomes more and more complex, cheating will in many ways 
I began to realize new possibilities opening up between the fields of
Cryptocurrencies allowed non-custodial exchange, without users having
Not your keys, Not your coins.
Bitcoin: A purely peer-to-peer version of electronic cash would allow
```

5. We repeat step 3 & 4 till we get the full messages,
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