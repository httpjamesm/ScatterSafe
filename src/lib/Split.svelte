<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";

    // @ts-ignore
    import _sodium from "libsodium-wrappers";

    // @ts-ignore
    import QRCode from "qrcode";

    import {
        PasswordInput,
        TextArea,
        Button,
        Form,
        FormGroup,
    } from "carbon-components-svelte";

    import Password from "carbon-icons-svelte/lib/Password.svelte";
    import DeploymentUnitTechnicalExecution from "carbon-icons-svelte/lib/DeploymentUnitTechnicalExecution.svelte";

    import { getDicewareWords, getHexPassword } from "./password";

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
            _sodium.crypto_pwhash_ALG_ARGON2ID13,
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

    const generateStrongPassword = async () => {
        password = await getDicewareWords(7);
    };

    const generateRandomPassword = async () => {
        password = await getHexPassword(6);
    }
</script>

<Form>
    <FormGroup>
        <TextArea
            labelText="Secret"
            placeholder="Crypto seed, TOTP secret, password, etc."
            bind:value={secret}
        />
    </FormGroup>
    <FormGroup>
        <PasswordInput
            labelText="Encryption Password"
            placeholder="Enter password..."
            bind:value={password}
        />
        <Button
            kind="tertiary"
            icon={Password}
            iconDescription="Generate a strong memorable password"
            on:click={generateStrongPassword}
        />
        <Button
            kind="tertiary"
            icon={DeploymentUnitTechnicalExecution}
            iconDescription="Generate a strong random password"
            on:click={generateRandomPassword}
        />
    </FormGroup>
    <FormGroup>
        <Button on:click={encryptSecret} style="margin-top: 1rem;"
            >Encrypt & Split</Button
        >
    </FormGroup>
    <div class="codes">
        {#each qrCodes as qrCode}
            <img class="code" src={qrCode} alt="QR code" />
        {/each}
    </div>
</Form>

<style lang="scss">
    .codes {
        display: flex;
        gap: 0.5rem;

        .code {
            height: 7rem;
            width: 7rem;
        }
    }
</style>
