<!--
Copyright: Anki Software, LLC and contributors
License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html
-->
<script lang="ts">
    import * as tr from "@generated/ftl";

<<<<<<< HEAD
    import DynamicallySlottable from "#lib/components/DynamicallySlottable.svelte";
    import HelpModal from "#lib/components/HelpModal.svelte";
    import Item from "#lib/components/Item.svelte";
    import SettingTitle from "#lib/components/SettingTitle.svelte";
    import SwitchRow from "#lib/components/SwitchRow.svelte";
    import TitledContainer from "#lib/components/TitledContainer.svelte";
    import { type HelpItem, HelpItemScheduler } from "#lib/components/types.js";
=======
    import DynamicallySlottable from "$lib/components/DynamicallySlottable.svelte";
    import Item from "$lib/components/Item.svelte";
    import Row from "$lib/components/Row.svelte";
    import SwitchRow from "$lib/components/SwitchRow.svelte";
    import TitledContainer from "$lib/components/TitledContainer.svelte";
>>>>>>> 36c4f39d7a9880063656981b7d1f35dd23a38007

    import ApplyScheduleModal from "./ApplyScheduleModal.svelte";
    import FsrsOptions from "./FsrsOptions.svelte";
    import GlobalLabel from "./GlobalLabel.svelte";
<<<<<<< HEAD
    import type { DeckOptionsState } from "./lib";
    import SchedulerOptions from "./SchedulerOptions.svelte";
=======
    import { type DeckOptionsState, ValueTab } from "./lib";
    import MaximumIntervalInputRow from "./MaximumIntervalInputRow.svelte";
    import SchedulerHelp from "./SchedulerHelp.svelte";
    import SchedulerOptions from "./SchedulerOptions.svelte";
    import SchedulerHelpButton from "./SchedulerHelpButton.svelte";
    import SchedulerRetention from "./SchedulerRetention.svelte";
    import { schedulerCardOrder } from "./scheduler-choice";
    import Warning from "./Warning.svelte";
>>>>>>> 36c4f39d7a9880063656981b7d1f35dd23a38007

    export let state: DeckOptionsState;
    export let study = false;
    export let api: Record<string, never>;

    export function onPresetChange() {
        desiredRetentionTabs[0] = new ValueTab(
            tr.deckConfigSharedPreset(),
            $config.desiredRetention,
            (value) => ($config.desiredRetention = value!),
            $config.desiredRetention,
            null,
        );
        effectiveDesiredRetention =
            $limits.desiredRetention ?? $config.desiredRetention;
    }

    const fsrs = state.fsrs;
    const config = state.currentConfig;
    const defaults = state.defaults;
    const limits = state.deckLimits;
    let schedulerHelp: SchedulerHelp;
    let desiredRetentionFocused = false;
    let desiredRetentionWarning = "";
    let retentionWarningClass = "";
    let effectiveDesiredRetention =
        $limits.desiredRetention ?? $config.desiredRetention;
    const desiredRetentionTabs: ValueTab[] = [
        new ValueTab(
            tr.deckConfigSharedPreset(),
            $config.desiredRetention,
            (value) => ($config.desiredRetention = value!),
            $config.desiredRetention,
            null,
        ),
        new ValueTab(
            tr.deckConfigDeckOnly(),
            $limits.desiredRetention ?? null,
            (value) => ($limits.desiredRetention = value ?? undefined),
            null,
            null,
        ),
    ];
    let newlyEnabled = false;
    $: modelCards = schedulerCardOrder($config);
    $: if (!$fsrs) {
        newlyEnabled = true;
    }
<<<<<<< HEAD

    const settings = {
        fsrs: {
            title: "FSRS",
            help: tr.deckConfigFsrsTooltip(),
            url: HelpPage.DeckOptions.fsrs,
            global: true,
        },
        // the desktop fork's scheduler choice (RWKV-Instant, RWKV-Curve, FSRS)
        review: {
            title: tr.deckConfigSchedulerReview(),
            help: [
                "### RWKV",
                tr.deckConfigRwkvDescription(),
                "### FSRS",
                tr.deckConfigFsrsDescription(),
            ].join("\n\n"),
            url: HelpPage.DeckOptions.fsrs,
        },
        dueDates: {
            title: tr.deckConfigSchedulerDueDates(),
            help: tr.deckConfigSchedulerDueDatesDescription(),
        },
        desiredRetention: {
            title: tr.deckConfigDesiredRetention(),
            help:
                tr.deckConfigDesiredRetentionTooltip() +
                "\n\n" +
                tr.deckConfigDesiredRetentionTooltip2(),
            sched: HelpItemScheduler.FSRS,
        },
        modelParams: {
            title: tr.deckConfigWeights(),
            help:
                tr.deckConfigWeightsTooltip2() +
                "\n\n" +
                tr.deckConfigComputeOptimalWeightsTooltip2(),
            sched: HelpItemScheduler.FSRS,
        },
        rescheduleCardsOnChange: {
            title: tr.deckConfigRescheduleCardsOnChange(),
            help: tr.deckConfigRescheduleCardsOnChangeTooltip(),
            sched: HelpItemScheduler.FSRS,
            global: true,
        },
        healthCheck: {
            title: tr.deckConfigHealthCheck(),
            help:
                tr.deckConfigAffectsEntireCollection() +
                "\n\n" +
                tr.deckConfigHealthCheckTooltip1() +
                "\n\n" +
                tr.deckConfigHealthCheckTooltip2(),
            sched: HelpItemScheduler.FSRS,
            global: true,
        },
    };
    const helpSections: HelpItem[] = Object.values(settings);

    let modal: Modal;
    let carousel: Carousel;

    function openHelpModal(index: number): void {
        modal.show();
        carousel.to(index);
    }
=======
>>>>>>> 36c4f39d7a9880063656981b7d1f35dd23a38007
</script>

<Row class="row-columns">
    <TitledContainer title={study ? tr.deckConfigStudyScheduling() : "Scheduler"}>
        <ApplyScheduleModal slot="header-actions" {state} />
        <SchedulerHelp
            title={"Scheduler"}
            showBadge={false}
            fsrs={$fsrs}
            bind:this={schedulerHelp}
        />
        <DynamicallySlottable slotHost={Item} {api}>
            <Item>
                <SwitchRow bind:value={$fsrs} defaultValue={false}>
                    <span class="scheduler-setting-label">
                        <GlobalLabel title={tr.deckConfigSchedulerEnableFsrs()} />
                        <SchedulerHelpButton
                            title={tr.deckConfigSchedulerEnableFsrs()}
                            on:click={() => schedulerHelp.open("fsrs")}
                        />
                    </span>
                </SwitchRow>
            </Item>

<<<<<<< HEAD
        {#if !$fsrs}
            <p class="scheduler-disabled">{tr.deckConfigSchedulerDisabled()}</p>
        {/if}

        <!-- Writes the preset's RWKV fields, which sync with the desktop fork. -->
        <SchedulerOptions
            {state}
            onSelect={() => fsrs.set(true)}
            openHelp={(key) => openHelpModal(Object.keys(settings).indexOf(key))}
        />

        {#if $fsrs}
            <FsrsOptions
                bind:this={fsrsOptionsComponent}
=======
            {#if !$fsrs}
                <p class="scheduler-disabled">{tr.deckConfigSchedulerDisabled()}</p>
            {/if}

            <SchedulerOptions
>>>>>>> 36c4f39d7a9880063656981b7d1f35dd23a38007
                {state}
                {study}
                onSelect={() => fsrs.set(true)}
                openHelp={(key) => schedulerHelp.open(key)}
            />

            <div
                class:study-line={study}
                class:combined-values={study}
                class:pass-through={!study}
            >
                {#if $fsrs || $config.rwkvReviewEnabled}
                    <Item>
                        <SchedulerRetention
                            {study}
                            bind:value={effectiveDesiredRetention}
                            defaultValue={defaults.desiredRetention}
                            tabs={desiredRetentionTabs}
                            bind:focused={desiredRetentionFocused}
                        >
                            <span>{tr.deckConfigDesiredRetention()}</span>
                            <SchedulerHelpButton
                                title={tr.deckConfigDesiredRetention()}
                                on:click={() => schedulerHelp.open("desiredRetention")}
                            />
                        </SchedulerRetention>
                    </Item>
                {/if}
                {#if study}
                    <Item>
                        <span>
                            {tr.schedulingMaximumInterval()}
                            <MaximumIntervalInputRow
                                inlineLabel={tr.schedulingMaximumInterval()}
                                defaultValue={defaults.maximumReviewInterval}
                                bind:value={$config.maximumReviewInterval}
                            />
                        </span>
                    </Item>
                {/if}
            </div>
            {#if $fsrs || $config.rwkvReviewEnabled}
                <Warning
                    warning={desiredRetentionWarning}
                    className={retentionWarningClass}
                />
            {/if}
            {#if study && $config.maximumReviewInterval < 180}
                <Warning
                    warning={tr.deckConfigTooShortMaximumInterval()}
                    className={$config.maximumReviewInterval < 50
                        ? "alert-danger"
                        : "alert-warning"}
                />
            {/if}
        </DynamicallySlottable>
    </TitledContainer>
</Row>

{#each modelCards as card (card)}
    {#if card === "rwkv"}
        <slot name="rwkv" />
    {:else if card === "fsrs" && ($fsrs || $config.rwkvReviewEnabled)}
        <!-- The simulator's embedded controls also need a slot-host context. -->
        <DynamicallySlottable slotHost={Item} api={{}}>
            <FsrsOptions
                {state}
                {study}
                {newlyEnabled}
                {desiredRetentionTabs}
                bind:effectiveDesiredRetention
                bind:desiredRetentionFocused
                bind:desiredRetentionWarning
                bind:retentionWarningClass
                openSchedulerHelp={(key) => schedulerHelp.open(key)}
                {onPresetChange}
            />
<<<<<<< HEAD
        {/if}
    </DynamicallySlottable>
</TitledContainer>

<style>
=======
        </DynamicallySlottable>
    {/if}
{/each}

<style>
    .pass-through {
        display: contents;
    }
    .combined-values {
        align-items: baseline;
    }
    .scheduler-setting-label {
        display: inline-flex;
        align-items: center;
        gap: 0.25rem;
    }

>>>>>>> 36c4f39d7a9880063656981b7d1f35dd23a38007
    .scheduler-disabled {
        color: var(--fg-subtle);
        font-size: 0.8rem;
        margin: 0.375rem 0;
    }
</style>
