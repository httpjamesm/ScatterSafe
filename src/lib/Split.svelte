<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";

    // @ts-ignore
    import _sodium from "libsodium-wrappers";

    // @ts-ignore
    import QRCode from "qrcode";

    import { PasswordInput, TextArea, Button } from "carbon-components-svelte";

    let secret = "";
    let password = "";

    let qrCodes: string[] = [];

    const encryptSecret = async () => {
        // derivekey with crypto pw function
        await _sodium.ready;

        const salt = _sodium.randombytes_buf(_sodium.crypto_pwhash_SALTBYTES);

        const saltB64 = _sodium.to_base64(
            salt,
            _sodium.base64_variants.URLSAFE_NO_PADDING
        );

        const key = _sodium.crypto_pwhash(
            _sodium.crypto_secretbox_KEYBYTES,
            password,
            salt,
            _sodium.crypto_pwhash_OPSLIMIT_INTERACTIVE,
            _sodium.crypto_pwhash_MEMLIMIT_INTERACTIVE,
            _sodium.crypto_pwhash_ALG_DEFAULT
        );

        // encrypt 3 times with different nonces

        qrCodes = [];

        const nonce = _sodium.randombytes_buf(
            _sodium.crypto_secretbox_NONCEBYTES
        );
        const nonceB64 = _sodium.to_base64(
            nonce,
            _sodium.base64_variants.URLSAFE_NO_PADDING
        );
        const encrypted = _sodium.crypto_secretbox_easy(secret, nonce, key);
        const encryptedB64 = _sodium.to_base64(
            encrypted,
            _sodium.base64_variants.URLSAFE_NO_PADDING
        );

        const everything = `${encryptedB64}:${nonceB64}:${saltB64}`;

        const splits: string[] = await invoke("do_split", {
            secret: everything,
        });

        for (const split of splits) {
            const qrCode = await QRCode.toDataURL(split);
            qrCodes = [...qrCodes, qrCode];
        }
    };
</script>

<form on:submit|preventDefault>
    <TextArea
        labelText="Secret"
        placeholder="Crypto seed, TOTP secret, password, etc."
        bind:value={secret}
    />
    <div style="margin-top: .5rem;">
        <PasswordInput
            labelText="Encryption Password"
            placeholder="Enter password..."
            bind:value={password}
        />
    </div>

    <Button on:click={encryptSecret}>Encrypt & Split</Button>
    <div class="codes">
        {#each qrCodes as qrCode}
            <img class="code" src={qrCode} alt="QR code" />
        {/each}
    </div>
</form>

<style lang="scss">
    .codes {
        display: flex;
        gap: .5rem;
    }
</style>