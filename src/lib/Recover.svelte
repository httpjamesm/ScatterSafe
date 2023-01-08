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
        Accordion,
        AccordionItem,
        InlineNotification,
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

    let fileUploader: FileUploader;

    let successMessage: string | undefined = undefined;
    let errorMessage: string | null = null;

    const doScan = async () => {
        let html5Qrcode = new Html5Qrcode("file-uploader");

        if (!files) {
            errorMessage = "No QR files selected";
            return;
        }

        if (files.length < 2) {
            errorMessage = "Please upload at least 2 QR files";
            return;
        }

        let splits: string[] = [];

        for (const file of files) {
            let data: any;
            try {
                data = await html5Qrcode.scanFile(file, true);
            } catch {
                errorMessage = "Unable to scan QR code. Please try again with a clearer image.";
                return;
            }

            if (data) {
                splits = [...splits, data];
            }
        }

        const recovered: string = await invoke("do_recover", {
            share1b64: splits[0],
            share2b64: splits[1],
        });

        if (recovered.length == 0) {
            errorMessage =
                "Unable to recover encrypted payload. Ensure you have the correct shares.";
            return;
        }

        const splitRecovered = recovered.split(":");

        if (splitRecovered.length !== 3) {
            errorMessage =
                "Unable to detect encryption metadata. Potentially corrupted or invalid QR codes.";
            return;
        }

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
            _sodium.crypto_pwhash_ALG_ARGON2ID13
        );

        let decrypted: Uint8Array | null = null;

        try {
            decrypted = _sodium.crypto_secretbox_open_easy(
                _sodium.from_base64(cipher),
                _sodium.from_base64(
                    nonce,
                    _sodium.base64_variants.URLSAFE_NO_PADDING
                ),
                key
            );
        } catch {
            errorMessage =
                "Unable to decrypt secret. Did you enter the correct password?";
            return;
        }

        if (!decrypted) {
            errorMessage =
                "Unable to decrypt secret. Did you enter the correct password?";
            return;
        }

        decodedSecret = new TextDecoder().decode(decrypted);

        errorMessage = null;
        successMessage = "Successfully recovered secret.";

        fileUploader.clearFiles();
    };

    const getPhoto = async () => {
        // take selfie
        const stream = await navigator.mediaDevices.getUserMedia({
            video: true,
        });

        let hiddenCanvas = document.createElement("canvas");
        let video = document.createElement("video");
        video.srcObject = stream;

        video.onloadedmetadata = async function (e) {
            video.play();

            // Get the exact size of the video element.
            let width = video.videoWidth;
            let height = video.videoHeight;

            // Context object for working with the canvas.
            const context = hiddenCanvas.getContext("2d");
            if (!context) return;

            // Set the canvas to the same dimensions as the video.
            hiddenCanvas.width = width;
            hiddenCanvas.height = height;

            // Draw a copy of the current frame from the video on the canvas.
            context.drawImage(video, 0, 0, width, height);

            // Get an image dataURL from the canvas.
            const imageDataURL = hiddenCanvas.toDataURL("image/png");

            const blob = await fetch(imageDataURL).then((r) => r.blob());

            const file = new File(
                [blob],
                `selfie-${Math.floor(Date.now() / 1000)}`,
                {
                    type: "image/png",
                }
            );

            files = [...files, file];

            // stop camera
            stream.getTracks().forEach((track) => track.stop());

            // remove video
            video.remove();
        };
    };

    onMount(init);
</script>

<Form>
    {#if errorMessage}
        <InlineNotification
            kind="error"
            title={"Error"}
            subtitle={errorMessage}
        />
    {:else if successMessage}
        <InlineNotification
            kind="success"
            title={"Success"}
            subtitle={successMessage}
        />
    {/if}

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
            bind:files
            status="complete"
            bind:this={fileUploader}
        />
        <Button on:click={getPhoto} kind="tertiary">Take photo</Button>
    </FormGroup>
    <FormGroup>
        <Button on:click={doScan}>Recover</Button>
        <Button
            kind="secondary"
            on:click={() => {
                fileUploader.clearFiles();
            }}>Clear Files</Button
        >
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
    <canvas id="file-uploader" />
</Form>

<Accordion>
    <AccordionItem title="What do I need to recover my secret?">
        <p>
            Thanks to the splitting algorithm, you only need 2 out of the 3
            generated QR codes and your original encryption password to recover
            the secret.
        </p>
    </AccordionItem>
    <AccordionItem title="I don't remember the encryption password.">
        <p>
            Unfortunately, without the original encryption password, you cannot
            recover your secret.
        </p>
    </AccordionItem>
</Accordion>
