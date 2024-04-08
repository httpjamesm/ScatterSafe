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
    SkeletonPlaceholder,
    Accordion,
    AccordionItem,
    InlineNotification,
    TextInput,
    MultiSelect,
  } from "carbon-components-svelte";

  import Password from "carbon-icons-svelte/lib/Password.svelte";
  import DeploymentUnitTechnicalExecution from "carbon-icons-svelte/lib/DeploymentUnitTechnicalExecution.svelte";

  import { getDicewareWords, getHexPassword } from "./password";

  import { jsPDF } from "jspdf";

  import JSZip from "jszip";

  import { message, save } from "@tauri-apps/api/dialog";
  import { writeBinaryFile } from "@tauri-apps/api/fs";

  import zxcvbn from "zxcvbn";

  let secret = "";
  let password = "";
  let label = "";

  let qrCodes: string[] = [];
  let splits: string[] = [];

  let loading = false;

  let successMessage: string | undefined = undefined;
  let errorMessage: string | null = null;

  let selectedExportTypes: string[] = [];

  const encryptSecret = async () => {
    if (!secret) {
      errorMessage = "You must enter a secret.";
      return;
    }

    if (!password) {
      errorMessage = "You must enter a password.";
      return;
    }

    loading = true;

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
      _sodium.crypto_pwhash_ALG_ARGON2ID13
    );

    // encrypt 3 times with different nonces

    qrCodes = [];

    const nonce = _sodium.randombytes_buf(_sodium.crypto_secretbox_NONCEBYTES);
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

    const everythingByteLength = new TextEncoder().encode(everything).length;

    if (everythingByteLength > 375) {
      errorMessage = "Secret is too long. Try a shorter secret.";
      loading = false;
      return;
    }

    splits = await invoke("do_split", {
      secret: everything,
    });

    for (const split of splits) {
      const qrCode = await QRCode.toDataURL(split, {
        errorCorrectionLevel: "L",
      });
      qrCodes = [...qrCodes, qrCode];
    }

    loading = false;

    errorMessage = null;
    successMessage =
      "Secret encrypted and split successfully. Save, print and distribute the QR codes.";
  };

  const generateStrongPassword = async () => {
    password = await getDicewareWords(7);
  };

  const generateRandomPassword = async () => {
    password = await getHexPassword(8);
  };

  const doExport = async () => {
    if (!selectedExportTypes.length) {
      errorMessage = "You must select at least one export type.";
      return;
    }

    loading = true;

    let zip = new JSZip();

    for (let i = 0; i < qrCodes.length; i++) {
      if (selectedExportTypes.includes("0")) {
        const doc = new jsPDF();

        doc.addImage(qrCodes[i], "PNG", 10, 10, 50, 50);
        doc.text(label, 35, 65, { align: "center" });

        const pdf = doc.output("arraybuffer");

        zip.file(`${label}-${i + 1}.pdf`, pdf);
      }

      if (selectedExportTypes.includes("1")) {
        const data = qrCodes[i].replace(/^data:image\/\w+;base64,/, "");
        const buf = Uint8Array.from(atob(data), (c) => c.charCodeAt(0));

        zip.file(`${label}-${i + 1}.png`, buf);
      }

      if (selectedExportTypes.includes("2")) {
        zip.file(`${label}-${i + 1}.txt`, splits[i]);
      }
    }

    const zipFile = await zip.generateAsync({ type: "arraybuffer" });

    const unixTimestampNow = Math.floor(Date.now() / 1000);

    const suggestedName = `${
      label || "scattersafe"
    }-backup-${unixTimestampNow}.zip`;

    const filePath = await save({
      filters: [
        {
          name: suggestedName,
          extensions: ["zip"],
        },
      ],
      defaultPath: suggestedName,
    });

    if (filePath) {
      await writeBinaryFile({
        contents: zipFile,
        path: filePath,
      });

      await message("Backups compressed and saved successfully.");
    }

    loading = false;
  };

  const defaultCrackTime = "less than a second";

  let crackTime = defaultCrackTime;
  let passwordScore = 0;

  $: if (password) {
    const res = zxcvbn(password);
    crackTime = res.crack_times_display.offline_slow_hashing_1e4_per_second;
    passwordScore = res.score;
  } else {
    crackTime = defaultCrackTime;
    passwordScore = 0;
  }
</script>

<Form>
  {#if errorMessage}
    <InlineNotification kind="error" title={"Error"} subtitle={errorMessage} />
  {:else if successMessage}
    <InlineNotification
      kind="success"
      title={"Success"}
      subtitle={successMessage}
    />
  {/if}

  <FormGroup>
    <TextArea
      labelText="Secret"
      placeholder="Crypto seed, TOTP secret, password, etc."
      bind:value={secret}
      disabled={loading}
    />
  </FormGroup>
  <FormGroup>
    <PasswordInput
      labelText="Encryption Password"
      placeholder="Enter password..."
      bind:value={password}
      disabled={loading}
    />
    <p
      class:bad-password={passwordScore <= 2}
      class:ok-password={passwordScore === 3}
      class:good-password={passwordScore > 3}
    >
      It would take <strong>{crackTime}</strong> to crack this password.
    </p>
    <br />
    <Button
      kind="tertiary"
      icon={Password}
      iconDescription="Generate a strong memorable password"
      on:click={generateStrongPassword}
      disabled={loading}
    />
    <Button
      kind="tertiary"
      icon={DeploymentUnitTechnicalExecution}
      iconDescription="Generate a strong random password"
      on:click={generateRandomPassword}
      disabled={loading}
    />
  </FormGroup>
  <FormGroup>
    <TextInput
      labelText="Label"
      placeholder="Bitcoin Wallet"
      bind:value={label}
    />
  </FormGroup>
  <FormGroup>
    <Button
      on:click={encryptSecret}
      style="margin-top: 1rem;"
      disabled={loading}>Encrypt & Split</Button
    >
  </FormGroup>
  <div class="codes">
    {#if loading && qrCodes.length === 0}
      {#each Array(3) as _, i}
        <SkeletonPlaceholder style="width: 7rem; height: 7rem;" />
      {/each}
    {/if}
    {#each qrCodes as qrCode}
      <img class="code" src={qrCode} alt="QR code" />
    {/each}
  </div>
  {#if qrCodes.length > 0}
    <FormGroup>
      <MultiSelect
        titleText="Export as..."
        label="Select formats..."
        items={[
          { id: "0", text: "PDF" },
          { id: "1", text: "JPEG" },
          { id: "2", text: "Text" },
        ]}
        bind:selectedIds={selectedExportTypes}
        disabled={loading}
      />
    </FormGroup>
    <FormGroup>
      <Button on:click={doExport} disabled={loading}>Export</Button>
    </FormGroup>
  {/if}
</Form>

<Accordion>
  <AccordionItem title="How does this work?">
    <p>
      ScatterSafe allows you to securely store reliable backups in multiple
      locations by using encryption and a splitting algorithm. The secret you
      enter into ScatterSafe will be encrypted with your password and split into
      3 different QR codes which you can print and store in different places. If
      you ever need to recover your secret, you only need 2 of the 3 QR codes
      and the encryption password.
    </p>
  </AccordionItem>
  <AccordionItem title="What's the safest way to use ScatterSafe?">
    <p>
      Ideally, you should load this app on an airgapped device or a device that
      will no longer connect to the Internet. While ScatterSafe does not make
      any outside connections, this ensures that even if you have an adversary
      who has loaded malware onto your device, they cannot send or receive your
      secret or password.
    </p>
  </AccordionItem>
  <AccordionItem title="Are my backups secure against adversaries?">
    <p>
      Yes, they should be. Backups are end-to-end encrypted with XSalsa20 and
      their keys are derived from your password using Argon2id. Thanks to
      secretbox architecture, these encrypted backups are also authenticated,
      meaning if an attacker were to try to tamper with the encrypted data, they
      would fail without the original password.
    </p>
  </AccordionItem>
  <AccordionItem title="Where should I store my backups?">
    <p>
      Store these backups in multiple places in spots that are hidden from plain
      sight but you can remember later on should you need to recover the secret.
      Ensure your backups are not stored in areas that are prone to flooding,
      fire, or other natural disasters.
    </p>
  </AccordionItem>
</Accordion>

<style lang="scss">
  .codes {
    display: flex;
    gap: 0.5rem;

    .code {
      height: 7rem;
      width: 7rem;
    }
  }

  .bad-password {
    color: var(--cds-danger-02);
  }

  .ok-password {
    color: #dd9912;
  }

  .good-password {
    color: var(--cds-support-02);
  }
</style>
