<script lang="ts">
	import { page } from '$app/state';
	import { getMass, deleteMass } from '$lib/api';
	import { goto } from '$app/navigation';
	import type { Mass } from '$lib/types';
	import MassCard from '$lib/components/MassCard.svelte';
	import { resolve } from '$app/paths';
	import type { Pathname } from '$app/types';
	import AsyncButton from '$lib/components/AsyncButton.svelte';

	let mass = $state<Mass | null>(null);
	let error = $state('');
	let loading = $state(true);

	let id = $derived(page.params.id);
	let redirectTo = $derived((page.url.searchParams.get('redirectTo') ?? '/masses') as Pathname);

	$effect(() => {
		if (!id) {
			if (redirectTo) goto(resolve(redirectTo));
		} else {
			loadData(id);
		}
	});

	async function loadData(currentId: string) {
		loading = true;
		error = '';
		try {
			mass = await getMass(currentId);
		} catch (e) {
			error = 'Failed to load mass.';
			console.error(e);
		} finally {
			loading = false;
		}
	}

	async function confirmDelete() {
		try {
			if (id) {
				await deleteMass(id);
			}
			await goto(resolve(redirectTo));
		} catch (e) {
			error = 'Failed to delete mass.';
			console.error(e);
		}
	}

	async function cancel() {
		await goto(resolve(redirectTo));
	}
</script>

{#if loading}
	<p>Loading...</p>
{:else if error}
	<p class="text-red-600">{error}</p>
{:else if mass}
	<h1 class="text-2xl font-bold mb-4">Delete Mass</h1>
	<p class="mb-2">Are you sure you want to delete the following mass?</p>

	<MassCard {mass} showActions={false} />

	<div class="flex space-x-4">
		<AsyncButton
			action={confirmDelete}
			class="bg-red-600 text-white px-4 py-2 rounded hover:bg-red-700"
		>
			Yes, Delete
		</AsyncButton>
		<AsyncButton
			action={cancel}
			class="bg-gray-300 dark:bg-gray-700 text-black dark:text-white px-4 py-2 rounded hover:bg-gray-400 dark:hover:bg-gray-600"
		>
			Cancel
		</AsyncButton>
	</div>
{:else}
	<p class="text-gray-500 italic">Mass not found.</p>
{/if}
