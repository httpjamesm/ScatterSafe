<script>
    import Recover from "$lib/Recover.svelte";
    import Split from "$lib/Split.svelte";
    import { Tabs, Tab, TabContent } from "carbon-components-svelte";
</script>

<div class="parent">
    <div class="child">
        <h1>ScatterSafe</h1>
        <Tabs>
            <Tab label="Split" />
            <Tab label="Recover" />
            <svelte:fragment slot="content">
                <TabContent>
                    <Split />
                </TabContent>
                <TabContent>
                    <Recover />
                </TabContent>
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
        }
    }
</style>
