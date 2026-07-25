<script lang="ts">
	import { untrack } from 'svelte';
	import { get } from 'svelte/store';
	import type { NewMass, Mass } from '$lib/types';
	import { formatTimestampLocal } from '$lib/utils';
	import { selectedUnit, convertFromKg, convertToKg, type MassUnit } from '$lib/stores/units';
	import { useAppData } from '$lib/queries';
	import AsyncButton from './AsyncButton.svelte';

	let {
		initial = {},
		onSubmit,
		onCancel,
		submitLabel = 'Submit',
		showCancel = false
	}: {
		initial?: Partial<Mass>;
		onSubmit: (data: NewMass) => Promise<void>;
		onCancel: () => Promise<void>;
		submitLabel?: string;
		showCancel?: boolean;
	} = $props();

	const appData = useAppData();

	let unit: MassUnit = $state(untrack(() => get(selectedUnit)));

	let massValue = $state(
		untrack(() => {
			const initialDisplayMass =
				initial.mass_kg !== undefined ? convertFromKg(initial.mass_kg, unit).toFixed(2) : '';
			return formatNumberString(initialDisplayMass);
		})
	);

	const hasInitialTimestamp = untrack(() => !!initial.created_at);
	let timestamp = $state(
		untrack(() =>
			initial.measurement_timestamp ? formatTimestampLocal(initial.measurement_timestamp) : ''
		)
	);

	let error = $state('');
	let timestampTouched = $state(false);
	let timer: ReturnType<typeof setInterval> | null = null;

	function formatNumberString(value: string): string {
		if (!value) return '';
		let formatted = value.replace(/[^0-9.]/g, '');
		formatted = formatted.replace(/(\..*?)\./g, '$1');
		formatted = formatted.replace(/^(-?)0+(\d)/, '$1$2');
		return formatted;
	}

	function handleUnitChange(e: Event) {
		const select = e.target as HTMLSelectElement;
		const newUnit = select.value as MassUnit;

		if (massValue && !isNaN(Number(massValue))) {
			const currentNum = Number(massValue);
			const asKg = convertToKg(currentNum, unit);
			const asNew = convertFromKg(asKg, newUnit);

			massValue = formatNumberString(asNew.toFixed(2));
		}

		unit = newUnit;
	}

	$effect(() => {
		if (!hasInitialTimestamp) {
			const updateTime = () => {
				if (!timestampTouched) {
					const now = new Date();
					const tzOffset = now.getTimezoneOffset() * 60000;
					const localISO = new Date(now.getTime() - tzOffset).toISOString().slice(0, 19);
					timestamp = localISO;
				}
			};

			updateTime();
			timer = setInterval(updateTime, 1000);

			return () => {
				if (timer) {
					clearInterval(timer);
					timer = null;
				}
			};
		}
	});

	const toISOStringIfDefined = (str: string | undefined) =>
		str ? new Date(str).toISOString() : undefined;

	async function submit() {
		error = '';

		if (!massValue) {
			error = 'Mass is required.';
			return;
		}

		const numericValue = Number(massValue);
		const payloadMassKg = convertToKg(numericValue, unit);

		const payload: NewMass = {
			mass_kg: payloadMassKg
		};

		if (timestampTouched || hasInitialTimestamp) {
			payload.measurement_timestamp = toISOStringIfDefined(timestamp || undefined);
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

<form onsubmit={(e) => e.preventDefault()} class="space-y-4 max-w-md">
	<div>
		<label for="mass" class="block font-medium">Mass</label>
		<div class="flex gap-2">
			<input
				id="mass"
				type="text"
				inputmode="numeric"
				bind:value={massValue}
				oninput={(e) => {
					const input = e.target as HTMLInputElement;
					massValue = formatNumberString(input.value.toString());
					input.value = massValue;
				}}
				class="grow p-2 border rounded text-green-800 dark:text-green-200"
				placeholder="0.0"
			/>

			<div class="relative w-24">
				<select
					value={unit}
					onchange={handleUnitChange}
					class="w-full h-full p-2 border rounded appearance-none bg-gray-50 dark:bg-gray-700 dark:border-gray-600 text-gray-900 dark:text-gray-100 pr-8"
				>
					<option value="kg">Kg</option>
					<option value="lbs">Lbs</option>
				</select>
				<div
					class="pointer-events-none absolute inset-y-0 right-0 flex items-center px-2 text-gray-700 dark:text-gray-300"
				>
					<svg class="fill-current h-4 w-4" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20"
						><path
							d="M9.293 12.95l.707.707L15.657 8l-1.414-1.414L10 10.828 5.757 6.586 4.343 8z"
						/></svg
					>
				</div>
			</div>
		</div>
	</div>

	<div>
		<label for="time" class="block font-medium">Timestamp</label>
		<input
			id="time"
			type="datetime-local"
			bind:value={timestamp}
			step="1"
			class="w-full p-2 border rounded"
			onfocus={handleTimestampFocusOrInput}
			oninput={handleTimestampFocusOrInput}
		/>
	</div>

	<div class="flex space-x-4">
		<AsyncButton
			type="submit"
			action={submit}
			class="bg-blue-600 text-white px-4 py-2 rounded hover:bg-blue-700"
			disabled={appData.isPending}
		>
			{submitLabel}
		</AsyncButton>
		{#if showCancel}
			<AsyncButton
				type="button"
				action={onCancel}
				class="bg-gray-300 dark:bg-gray-700 text-black dark:text-white px-4 py-2 rounded hover:bg-gray-400 dark:hover:bg-gray-600"
			>
				Cancel
			</AsyncButton>
		{/if}
	</div>
	{#if error}
		<p class="text-red-600 font-medium">{error}</p>
	{/if}
</form>
