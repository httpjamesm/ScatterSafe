# ScatterSafe

We all have important secrets we need to secure from prying eyes, but also ensure we have access to them in the event of an emergency. Stuff like crypto seeds, backup TOTP secrets for essential accounts, and other sensitive information. ScatterSafe is a simple, secure, and easy to use solution for storing this information.

This project came into existence because another app, called [Superbacked](https://superbacked.com), is charging at least $149 USD for a similar service, which simply doesn't make sense. As well, this project is free and open-source, so anyone can inspect the code and ensure it's not doing anything malicious.

## The Problem

Many trivial ways of storing this information that may come to mind are flawed.

### Password Managers

Password managers are secure thanks to their end-to-end encryption, but if you lose that decryption password, all your data is lost forever. There is also the risk of spyware viewing the decrypted contents of your password manager (while it's unlocked and the key is in memory), or a malicious password manager app.

### Pen and Paper

Pen and paper is a simple solution, but it's not secure. If someone finds your notes, they can read them. If you lose your notes, they're gone forever.

### USB Drives

USB drives by default are not encrypted, and if you lose that drive, you also lose that important data.

## The Solution

ScatterSafe uses the [Shamir secret sharing algorithm](https://en.wikipedia.org/wiki/Shamir%27s_Secret_Sharing) in conjunction with [XSalsa20](https://en.wikipedia.org/wiki/Salsa20) end-to-end [encryption](https://en.wikipedia.org/wiki/Encryption).

### How It Works

ScatterSafe will ask you for a secret and a password.

The secret is encrypted with your password using XSalsa20.

ScatterSafe creates 3 QR codes from the split encrypted secret. These QR codes can be printed and stored in different locations.

If you ever need to access your secret, you can use any 2 of the 3 QR codes to reconstruct the encrypted secret. You can then decrypt the secret with your password.

This is both a **secure** and **reliable** way of backing up important information because an attacker would have to know where to find at least 2 of your 3 QR codes and the password used to encrypt the original secret. If you for some reason cannot access all 3, maybe due to a natural disaster, you can still access your secret by using the 2 QR codes you have.
