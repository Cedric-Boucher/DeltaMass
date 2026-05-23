<script lang="ts">
	import { page } from '$app/state';
	import { getMass, updateMass } from '$lib/api';
	import { goto } from '$app/navigation';
	import MassForm from '$lib/components/MassForm.svelte';
	import type { NewMass, Mass } from '$lib/types';
	import { resolve } from '$app/paths';
	import type { Pathname } from '$app/types';

	let mass = $state<Mass | null>(null);

	let id = $derived(page.params.id);
	let redirectTo = $derived((page.url.searchParams.get('redirectTo') ?? '/masses') as Pathname);

	$effect(() => {
		if (id) {
			loadData(id);
		} else {
			if (redirectTo) goto(resolve(redirectTo));
		}
	});

	async function loadData(currentId: string) {
		mass = null;
		try {
			mass = await getMass(currentId);
		} catch (e) {
			console.error('Failed to load mass:', e);
		}
	}

	async function handleUpdate(data: NewMass) {
		if (id) {
			await updateMass(id, data);
		}
		goto(resolve(redirectTo));
	}

	function cancel() {
		goto(resolve(redirectTo));
	}
</script>

{#if mass}
	<h1 class="text-2xl font-bold mb-4">Edit Mass</h1>
	<MassForm
		initial={mass}
		onSubmit={handleUpdate}
		onCancel={cancel}
		submitLabel="Save Changes"
		showCancel={true}
	/>
{:else}
	<p>Loading...</p>
{/if}
