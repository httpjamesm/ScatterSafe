<script lang="ts">
    import { browser } from "$app/environment";
    import { invoke } from "@tauri-apps/api/tauri";

    // @ts-ignore
    import _sodium from "libsodium-wrappers";

    import {
        FileUploader,
        PasswordInput,
        Button,
        TextArea,
        Form,
        FormGroup,
    } from "carbon-components-svelte";

    let Html5Qrcode: any;
    import { onMount } from "svelte";

    const init = async () => {
        if (browser) {
            Html5Qrcode = (await import("html5-qrcode")).Html5Qrcode;
        }
    };

    let password = "";

    let files: File[];

    let decodedSecret = "";

    const doScan = async (e: any) => {
        let html5Qrcode = new Html5Qrcode("file-uploader");

        if (!files) {
            return;
        }

        if (files.length < 2) {
            return;
        }

        let splits: string[] = [];

        for (const file of files) {
            const data = await html5Qrcode.scanFile(file, true);

            if (data) {
                splits = [...splits, data];
            }
        }

        const recovered: string = await invoke("do_recover", {
            share1b64: splits[0],
            share2b64: splits[1],
        });

        const splitRecovered = recovered.split(":");

        const cipher = splitRecovered[0];
        const nonce = splitRecovered[1];
        const salt = splitRecovered[2];

        const key = _sodium.crypto_pwhash(
            _sodium.crypto_secretbox_KEYBYTES,
            password,
            _sodium.from_base64(
                salt,
                _sodium.base64_variants.URLSAFE_NO_PADDING
            ),
            _sodium.crypto_pwhash_OPSLIMIT_INTERACTIVE,
            _sodium.crypto_pwhash_MEMLIMIT_INTERACTIVE,
            _sodium.crypto_pwhash_ALG_ARGON2ID13,
        );

        const decrypted = _sodium.crypto_secretbox_open_easy(
            _sodium.from_base64(cipher),
            _sodium.from_base64(
                nonce,
                _sodium.base64_variants.URLSAFE_NO_PADDING
            ),
            key
        );

        decodedSecret = new TextDecoder().decode(decrypted);
    };

    onMount(init);
</script>

<Form>
    <FormGroup>
        <PasswordInput
            labelText="Encryption Password"
            placeholder="Enter password..."
            bind:value={password}
        />
    </FormGroup>
    <FormGroup>
        <FileUploader
            accept={[".jpg", ".png"]}
            multiple
            labelTitle="Upload QR Codes"
            labelDescription="Only .jpg and .png files are accepted."
            buttonLabel="Add files"
            id="file-uploader"
            bind:files
            status="complete"
        />
    </FormGroup>
    <FormGroup>
        <Button on:click={doScan}>Recover</Button>
    </FormGroup>
    {#if decodedSecret}
        <FormGroup>
            <TextArea
                value={decodedSecret}
                labelText="Recovered Secret"
                disabled
            />
        </FormGroup>
    {/if}
</Form>
