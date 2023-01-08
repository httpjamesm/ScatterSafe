# ScatterSafe

We all have important secrets we need to secure from prying eyes, yet we must ensure we have access to them in the event of an emergency. Crypto seeds, backup TOTP secrets for essential accounts, among other sensitive information. ScatterSafe is a simple, secure, and easy to use solution for storing this information.

This project exists because another company, called [Superbacked](https://superbacked.com), is charging a minimum of $149 USD for a similar service, which simply doesn't make sense. Furthermore, unlike Superbacked, this project is free and open-source, so anyone can inspect the code and ensure that it's not malicious.

![ScatterSafe Split Light](https://cdn.horizon.pics/yVfDx4M6TFDXc9P31W6bqx7CeEoFVl.png)

![ScatterSafe Recover Light](https://cdn.horizon.pics/v3I9Jls7pqKhLP46P60sgxsETBxHrt.png)

![ScatterSafe Split Dark](https://cdn.horizon.pics/W6mLKRPZdFeIplLQddkCrzLBiKMea0.png)

![ScatterSafe Recover Dark](https://cdn.horizon.pics/4GlKXV1zFEWZTPjpDscsbSJqOozMnJ.png)

## The Problem

Many trivial methods of storing this information that may come to mind are flawed.

### Password Managers

Password managers are secure thanks to their end-to-end encryption, but if you lose that decryption password, all your data is lost forever. There is also the risk of spyware viewing the decrypted contents of your password manager (while it's unlocked and the key is in memory), or a malicious password manager app.

### Pen and Paper

Pen and paper is a simple solution, but it's not secure. If someone finds your notes, they can read them. If you lose your notes, they're gone forever.

### USB Drives

USB drives by default are not encrypted. Regardless of if the drive encrypted, you could lose the drive, and your important data with it.

## The Solution

ScatterSafe uses the [Shamir secret sharing algorithm](https://en.wikipedia.org/wiki/Shamir%27s_Secret_Sharing) in conjunction with [XSalsa20](https://en.wikipedia.org/wiki/Salsa20) end-to-end [encryption](https://en.wikipedia.org/wiki/Encryption).

### How It Works

ScatterSafe will request a secret and a password.

The secret is encrypted with your password using XSalsa20.

ScatterSafe creates 3 QR codes from the split encrypted secret. These QR codes can be printed and stored in different locations.

If you ever need to access your secret, you can use any 2 of the 3 QR codes to reconstruct the encrypted secret. You can then decrypt the secret with your password.

This is both a **secure** and **reliable** way of backing up important information because an attacker would have to know where to find at least 2 of your 3 QR codes, along with the password used to encrypt the original secret. If for some reason you cannot access all 3, perhaps due to a natural disaster, you can still access your secret by using the remaining QR codes.
