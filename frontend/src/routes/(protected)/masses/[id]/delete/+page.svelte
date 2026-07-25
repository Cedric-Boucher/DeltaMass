<script lang="ts">
	import { page } from '$app/state';
	import { goto } from '$app/navigation';
	import { useAppData, useDeleteMass } from '$lib/queries';
	import MassCard from '$lib/components/MassCard.svelte';
	import { resolve } from '$app/paths';
	import type { Pathname } from '$app/types';
	import AsyncButton from '$lib/components/AsyncButton.svelte';

	const appData = useAppData();
	const deleteMassMutation = useDeleteMass();

	let error = $state('');

	let id = $derived(page.params.id);
	let numericId = $derived(Number(id));
	let redirectTo = $derived((page.url.searchParams.get('redirectTo') ?? '/masses') as Pathname);

	let mass = $derived(
		appData.data?.masses.find((m) => m.id === numericId || String(m.id) === id) ?? null
	);

	$effect(() => {
		if (!id) {
			if (redirectTo) goto(resolve(redirectTo));
		}
	});

	async function confirmDelete() {
		error = '';
		try {
			if (id) {
				await deleteMassMutation.mutateAsync(id);
			}
			await goto(resolve(redirectTo));
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to delete mass.';
			console.error(e);
		}
	}

	async function cancel() {
		await goto(resolve(redirectTo));
	}
</script>

{#if appData.isPending}
	<p>Loading...</p>
{:else if appData.isError}
	<p class="text-red-600">Failed to load mass data.</p>
{:else if mass}
	<h1 class="text-2xl font-bold mb-4">Delete Mass</h1>

	{#if error}
		<p class="text-red-600 mb-4">{error}</p>
	{/if}

	<p class="mb-2">Are you sure you want to delete the following mass?</p>

	<MassCard {mass} showActions={false} />

	<div class="flex space-x-4 mt-4">
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
