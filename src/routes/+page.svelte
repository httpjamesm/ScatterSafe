<script lang="ts">
    import Recover from "$lib/Recover.svelte";
    import Split from "$lib/Split.svelte";
    import { Tabs, Tab, TabContent } from "carbon-components-svelte";
    import { appWindow } from "@tauri-apps/api/window";
    import { emit, listen } from "@tauri-apps/api/event";
    import type { UnlistenFn } from "@tauri-apps/api/event";
    import { onMount } from "svelte";

    const checkTheme = async () => {
        // check system theme and change html
        const theme = await appWindow.theme();

        document.documentElement.setAttribute(
            "theme",
            theme === "light" ? "light" : "g100"
        );

        await appWindow.onThemeChanged(() => {
            checkTheme();
        });
    };

    onMount(() => {
        checkTheme();
    });
</script>

<div class="parent">
    <div class="child">
        <div class="header">
            <h1>ScatterSafe</h1>
        </div>
        <Tabs>
            <Tab label="Split" />
            <Tab label="Recover" />
            <svelte:fragment slot="content">
                <div class="scrollable">
                    <TabContent>
                        <Split />
                    </TabContent>
                    <TabContent>
                        <Recover />
                    </TabContent>
                </div>
            </svelte:fragment>
        </Tabs>
    </div>
</div>

<style lang="scss">
    .parent {
        width: 100vw;
        height: 100vh;
        display: flex;
        justify-content: center;
        align-items: center;

        .child {
            width: 30rem;

            overflow-y: hidden;

            .scrollable {
                overflow-y: scroll;
                overflow-x: hidden;
                height: calc(100vh - 120px);

                &::-webkit-scrollbar {
                    display: none; /* Safari and Chrome */
                }
            }
        }
    }
</style>
