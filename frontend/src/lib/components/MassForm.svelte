<script lang="ts">
    import { onDestroy, onMount } from 'svelte';
    import type { NewMass, Mass } from '$lib/types';
    import { get, writable } from 'svelte/store';
    import { formatTimestampLocal } from '$lib/utils';
    import { selectedUnit, convertFromKg, convertToKg, type MassUnit } from '$lib/stores/units';

    export let initial: Partial<Mass> = {};
    export let onSubmit: (data: NewMass) => Promise<void>;
    export let onCancel: () => void;
    export let submitLabel = 'Submit';
    export let showCancel: boolean = false;

    let unit: MassUnit = $selectedUnit;

    const initialDisplayMass = initial.mass_kg !== undefined 
        ? convertFromKg(initial.mass_kg, unit).toFixed(2)
        : '';

    const massValue = writable(formatNumberString(initialDisplayMass));

    const hasInitialTimestamp = !!initial.measurement_timestamp;
    const timestamp = writable(initial.measurement_timestamp ? formatTimestampLocal(initial.measurement_timestamp) : '');

    let error = '';
    let timestampTouched = false;
    let timer: ReturnType<typeof setInterval> | null = null;

    function formatNumberString(value: string): string {
        if (!value) return "";
        let formatted = value.replace(/[^0-9.]/g, "");
        formatted = formatted.replace(/(\..*?)\./g, "$1");
        formatted = formatted.replace(/^(-?)0+(\d)/, "$1$2");
        return formatted;
    }

    function handleUnitChange(e: Event) {
        const select = e.target as HTMLSelectElement;
        const newUnit = select.value as MassUnit;
        const currentStr = get(massValue);

        if (currentStr && !isNaN(Number(currentStr))) {
            const currentNum = Number(currentStr);
            const asKg = convertToKg(currentNum, unit);
            const asNew = convertFromKg(asKg, newUnit);

            massValue.set(formatNumberString(asNew.toFixed(2)));
        }

        unit = newUnit;
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

        const $massValue = get(massValue);
        const $timestamp = get(timestamp);

        if (!$massValue) {
            error = 'Mass is required.';
            return;
        }

        const numericValue = Number($massValue);
        const payloadMassKg = convertToKg(numericValue, unit);

        const payload: NewMass = {
            mass_kg: payloadMassKg,
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
        <div class="flex gap-2">
            <input
                id="mass"
                type="text"
                inputmode="numeric"
                bind:value={$massValue}
                on:input={
                    (e) => {
                        const input = e.target as HTMLInputElement;
                        massValue.set(formatNumberString(input.value.toString()));
                        input.value = get(massValue);
                    }
                }
                class="grow p-2 border rounded text-green-800 dark:text-green-200"
                placeholder="0.0"
            />

            <div class="relative w-24">
                <select
                    value={unit}
                    on:change={handleUnitChange}
                    class="w-full h-full p-2 border rounded appearance-none bg-gray-50 dark:bg-gray-700 dark:border-gray-600 text-gray-900 dark:text-gray-100 pr-8"
                >
                    <option value="kg">Kg</option>
                    <option value="lbs">Lbs</option>
                </select>
                <div class="pointer-events-none absolute inset-y-0 right-0 flex items-center px-2 text-gray-700 dark:text-gray-300">
                    <svg class="fill-current h-4 w-4" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20"><path d="M9.293 12.95l.707.707L15.657 8l-1.414-1.414L10 10.828 5.757 6.586 4.343 8z"/></svg>
                </div>
            </div>
        </div>
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
