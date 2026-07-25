<script lang="ts">
	import { goto } from '$app/navigation';
	import { untrack } from 'svelte';
	import MassCard from '$lib/components/MassCard.svelte';
	import { resolve } from '$app/paths';
	import AsyncButton from '$lib/components/AsyncButton.svelte';
	import { useAppData } from '$lib/queries';

	const appData = useAppData();

	let masses = $derived(appData.data?.masses ?? []);

	const BATCH_SIZE = 50;
	const ESTIMATED_ITEM_HEIGHT = 80;

	let limit = $state(BATCH_SIZE);

	function loadMore() {
		if (limit < masses.length) {
			limit += BATCH_SIZE;
			setTimeout(loadMore, 0);
		}
	}

	$effect(() => {
		if (masses.length > 0) {
			untrack(() => {
				limit = BATCH_SIZE;
				if (typeof window !== 'undefined') window.scrollTo(0, 0);
				setTimeout(loadMore, 0);
			});
		}
	});

	let visibleMasses = $derived(masses.slice(0, limit));
	let remainingCount = $derived(Math.max(0, masses.length - limit));
	let phantomHeight = $derived(remainingCount * ESTIMATED_ITEM_HEIGHT);
</script>

<h1 class="text-2xl font-bold mb-4">Masses</h1>

<div class="mb-6 flex items-center gap-4">
	<AsyncButton
		class="bg-blue-600 text-white px-4 py-2 rounded hover:bg-blue-700"
		action={async () => await goto(resolve('/masses/new'))}
	>
		+ New Mass
	</AsyncButton>
</div>

{#if appData.isPending}
	<p>Loading masses...</p>
{:else if appData.isError}
	<p class="text-red-600">Failed to load masses.</p>
{:else if visibleMasses.length}
	<ul class="space-y-4">
		{#each visibleMasses as mass (mass.id)}
			<MassCard {mass} showActions={true} />
		{/each}

		<div style="height: {phantomHeight}px; width: 100%"></div>
	</ul>
{:else}
	<p class="text-gray-500 italic">No masses found.</p>
{/if}
