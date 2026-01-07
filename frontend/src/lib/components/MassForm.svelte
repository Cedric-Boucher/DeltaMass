<script lang="ts">
    import { onDestroy, onMount } from 'svelte';
    import type { NewMass, Mass } from '$lib/types';
    import { get, writable } from 'svelte/store';
    import { formatTimestampLocal } from '$lib/utils';

    export let initial: Partial<Mass> = {};
    export let onSubmit: (data: NewMass) => Promise<void>;
    export let onCancel: () => void;
    export let submitLabel = 'Submit';
    export let showCancel: boolean = false;

    const mass_kg = writable(formatNumberString(initial.mass_kg?.toString() ?? ''));

    const hasInitialTimestamp = !!initial.measurement_timestamp;
    const timestamp = writable(initial.measurement_timestamp ? formatTimestampLocal(initial.measurement_timestamp) : '');

    let error = '';

    let timestampTouched = false;
    let timer: ReturnType<typeof setInterval> | null = null;

    function formatNumberString(value: string): string {
        if (!value) {
            console.log("format number received: ", value);
            return "";
        }

        // Step 1: keep only digits, and decimal point(s)
        let formatted = value.replace(/[^0-9.]/g, "");
        // Step 2: remove all negative symbols except the first character
        // formatted = formatted.replace(/(?!^)-/g, "");
        // Step 3: remove extra decimal points (keep only the first one)
        formatted = formatted.replace(/(\..*?)\./g, "$1");
        // Step 4: trim leading zeros (but preserve "0" and "-0")
        formatted = formatted.replace(/^(-?)0+(\d)/, "$1$2");

        console.log("Formatted number: ", formatted);

        return formatted;
    }

    onMount(async () => {
        if (!hasInitialTimestamp) {
            const updateTime = () => {
                if (!timestampTouched) {
                    const now = new Date();
                    const tzOffset = now.getTimezoneOffset() * 60000;
                    const localISO = new Date(now.getTime() - tzOffset).toISOString().slice(0, 19);
                    timestamp.set(localISO);
                }
            };
            updateTime();
            timer = setInterval(updateTime, 1000);
        }

        onDestroy(() => {
            if (timer) clearInterval(timer);
        });
    });

    const toISOStringIfDefined = (str: string | undefined) =>
        str ? new Date(str).toISOString() : undefined;

    async function submit() {
        error = '';

        const $mass_kg = get(mass_kg);
        const $timestamp = get(timestamp);

        if (!$mass_kg) {
            error = 'Mass is required.';
            return;
        }

        const payload: NewMass = {
            mass_kg: Number($mass_kg),
        };

        if (timestampTouched || hasInitialTimestamp) {
            payload.measurement_timestamp = toISOStringIfDefined($timestamp || undefined);
        }

        try {
            await onSubmit(payload);
        } catch (e) {
            error = 'Failed to submit mass.';
            console.error(e);
        }
    }

    function handleTimestampFocusOrInput() {
        timestampTouched = true;
        if (timer) {
            clearInterval(timer);
            timer = null;
        }
    }
</script>

<form on:submit|preventDefault={submit} class="space-y-4 max-w-md">
    <div>
        <label for="mass" class="block font-medium">Mass</label>
        <input
            id="mass"
            type="text"
            inputmode="numeric"
            bind:value={$mass_kg}
            on:input={
                (e) => {
                    const input = e.target as HTMLInputElement;
                    mass_kg.set(formatNumberString(input.value.toString()));
                    input.value = get(mass_kg);
                }
            }
            class="w-full p-2 border rounded text-green-800 dark:text-green-200"
        />
    </div>

    <div>
        <label for="time" class="block font-medium">Timestamp</label>
        <input
            id="time"
            type="datetime-local"
            bind:value={$timestamp}
            step="1"
            class="w-full p-2 border rounded"
            on:focus={handleTimestampFocusOrInput}
            on:input={handleTimestampFocusOrInput}
        />
    </div>

    <div class="flex space-x-4">
        <button type="submit" class="bg-blue-600 text-white px-4 py-2 rounded hover:bg-blue-700">
            {submitLabel}
        </button>
        {#if showCancel}
            <button
                type="button"
                on:click={onCancel}
                class="bg-gray-300 dark:bg-gray-700 text-black dark:text-white px-4 py-2 rounded hover:bg-gray-400 dark:hover:bg-gray-600"
            >
                Cancel
            </button>
        {/if}
    </div>
    {#if error}
        <p class="text-red-600">{error}</p>
    {/if}
</form>
