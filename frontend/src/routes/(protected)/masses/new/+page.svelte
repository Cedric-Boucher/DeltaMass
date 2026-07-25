<script lang="ts">
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';
	import { useCreateMass } from '$lib/queries';
	import MassForm from '$lib/components/MassForm.svelte';
	import type { NewMass } from '$lib/types';

	const createMass = useCreateMass();

	async function handleSubmit(data: NewMass) {
		await createMass.mutateAsync(data);
		await goto(resolve('/masses'));
	}

	async function cancel() {
		await goto(resolve('/masses'));
	}
</script>

<h1 class="text-2xl font-bold mb-4">New Mass</h1>

<MassForm onSubmit={handleSubmit} onCancel={cancel} submitLabel="Create" showCancel={true} />
