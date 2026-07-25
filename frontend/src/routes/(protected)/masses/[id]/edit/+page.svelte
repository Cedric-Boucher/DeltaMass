<script lang="ts">
	import { page } from '$app/state';
	import { goto } from '$app/navigation';
	import { useAppData, useUpdateMass } from '$lib/queries';
	import MassForm from '$lib/components/MassForm.svelte';
	import type { NewMass } from '$lib/types';
	import { resolve } from '$app/paths';
	import type { Pathname } from '$app/types';

	const appData = useAppData();
	const updateMassMutation = useUpdateMass();

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

	async function handleUpdate(data: NewMass) {
		if (id) {
			await updateMassMutation.mutateAsync({ id, data });
		}
		await goto(resolve(redirectTo));
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
	<h1 class="text-2xl font-bold mb-4">Edit Mass</h1>
	<MassForm
		initial={mass}
		onSubmit={handleUpdate}
		onCancel={cancel}
		submitLabel="Save Changes"
		showCancel={true}
	/>
{:else}
	<p class="text-gray-500 italic">Mass not found.</p>
{/if}
