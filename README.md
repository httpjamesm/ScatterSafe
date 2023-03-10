# ScatterSafe

We all have sensitive secrets we need to secure from prying eyes. Yet, we must ensure we have access to them in an emergency—crypto seeds, TOTP secrets for essential accounts, among other sensitive information. ScatterSafe is a simple, secure, and easy-to-use solution for storing this information.

This project exists because another company, named [Superbacked](https://superbacked.com), charges a minimum of **$149** for a similar service, which doesn't make sense. Furthermore, unlike Superbacked, this project is free and open-source, so anyone can inspect the code and ensure it's not malicious.

![ScatterSafe Split Light](https://files.horizon.pics/aa2f658b-6a7e-4a46-8846-821b40716a85?a=1&mime1=image&mime2=png)

![ScatterSafe Recover Light](https://files.horizon.pics/b4dee93a-50d8-477f-81c2-f5b1d1088c12?a=1&mime1=image&mime2=png)

![ScatterSafe Split Dark](https://files.horizon.pics/0efb38eb-a0da-4b53-99bd-b7c040cfc6c7?a=1&mime1=image&mime2=png)

![ScatterSafe Recover Dark](https://files.horizon.pics/e0ec9ab7-3879-4ffb-8cc6-71fa5ec5d42d?a=1&mime1=image&mime2=png)

## The Problem

Many of the known methods of storing this data that come to mind are flawed.

### Password Managers

Password managers are secure thanks to their end-to-end encryption, but if you lose that decryption password, all your data is lost forever. There is also the risk of spyware viewing the decrypted contents of your password manager (while it's unlocked and the key is in memory) or a malicious password manager app.

### Pen and Paper

Pen and paper is a simple solution, but it's not secure. If someone finds your notes, someone else now possesses your secrets. Attempting to prevent this while also remembering where you left them can be difficult and result in them being misplaced and lost forever.

### USB Drives

USB drives, by default, are not encrypted. Regardless of whether the drive is encrypted, you could lose it along with your critical data.

## The Solution

ScatterSafe uses the [Shamir secret sharing algorithm](https://en.wikipedia.org/wiki/Shamir%27s_Secret_Sharing) in conjunction with [XSalsa20](https://en.wikipedia.org/wiki/Salsa20) end-to-end [encryption](https://en.wikipedia.org/wiki/Encryption).

### How It Works

ScatterSafe will request a secret and a password.

The secret is encrypted with your password using XSalsa20.

ScatterSafe creates 3 QR codes from the split encrypted secret. These QR codes can be printed and stored in different locations.

If you ever need to access your secret, you can use 2 of the 3 QR codes to reconstruct the encrypted secret. You can then decrypt the secret with your password.

This method provides both a **secure** and **reliable** way of backing up important information. An attacker must know where to find at least 2 of your 3 QR codes and the password used to encrypt the original secret. If for some reason, you cannot access all three due to a natural disaster or similar, you can still access your secret by using the remaining QR codes.
