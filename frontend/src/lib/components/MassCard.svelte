<script lang="ts">
	import type { Mass } from '$lib/types';
	import { formatTimestampLocalForDisplay } from '$lib/utils';
	import { goto } from '$app/navigation';
	import { page } from '$app/state';

	import { selectedUnit, formatMass, UNIT_LABELS } from '$lib/stores/units';
	import { resolve } from '$app/paths';
	import AsyncButton from './AsyncButton.svelte';

	let {
		mass,
		showActions = true,
		onEdit = null,
		onDelete = null
	}: {
		mass: Mass;
		showActions?: boolean;
		onEdit?: (() => void) | null;
		onDelete?: (() => void) | null;
	} = $props();

	async function handleEdit() {
		if (onEdit) {
			onEdit();
		} else {
			await goto(
				resolve(`/masses/${mass.id}/edit?redirectTo=${encodeURIComponent(page.url.pathname)}`)
			);
		}
	}

	async function handleDelete() {
		if (onDelete) {
			onDelete();
		} else {
			await goto(
				resolve(`/masses/${mass.id}/delete?redirectTo=${encodeURIComponent(page.url.pathname)}`)
			);
		}
	}
</script>

<li class="bg-white dark:bg-gray-800 shadow rounded p-4 flex justify-between items-start gap-4">
	<div>
		<div class="text-2xl font-bold">
			{formatMass(mass.mass_kg, $selectedUnit)}
			{UNIT_LABELS[$selectedUnit]}
		</div>
		<div class="text-gray-500 text-sm">
			{formatTimestampLocalForDisplay(mass.measurement_timestamp)}
		</div>
	</div>

	{#if showActions}
		<div class="flex flex-col gap-2">
			<AsyncButton
				class="bg-blue-600 text-white px-4 py-2 rounded hover:bg-blue-700"
				action={handleEdit}>Edit</AsyncButton
			>
			<AsyncButton
				class="bg-red-600 text-white px-4 py-2 rounded hover:bg-red-700"
				action={handleDelete}>Delete</AsyncButton
			>
		</div>
	{/if}
</li>
