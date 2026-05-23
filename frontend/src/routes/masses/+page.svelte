<script lang="ts">
	import { goto } from '$app/navigation';
	import { untrack } from 'svelte';
	import type { Mass } from '$lib/types';
	import MassCard from '$lib/components/MassCard.svelte';
	import { resolve } from '$app/paths';

	let { data }: { data: { masses: Mass[] } } = $props();

	const BATCH_SIZE = 100;
	const ESTIMATED_ITEM_HEIGHT = 80;

	let limit = $state(BATCH_SIZE);

	function loadMore() {
		if (limit < data.masses.length) {
			limit += BATCH_SIZE;
			setTimeout(loadMore, 0);
		}
	}

	$effect(() => {
		if (data.masses) {
			untrack(() => {
				limit = BATCH_SIZE;
				if (typeof window !== 'undefined') window.scrollTo(0, 0);
				setTimeout(loadMore, 0);
			});
		}
	});

	let visibleMasses = $derived(data.masses.slice(0, limit));
	let remainingCount = $derived(Math.max(0, data.masses.length - limit));
	let phantomHeight = $derived(remainingCount * ESTIMATED_ITEM_HEIGHT);
</script>

<h1 class="text-2xl font-bold mb-4">Masses</h1>

<div class="mb-6 flex items-center gap-4">
	<button
		class="bg-blue-600 text-white px-4 py-2 rounded hover:bg-blue-700"
		onclick={() => goto(resolve('/masses/new'))}
	>
		+ New Mass
	</button>
</div>

{#if visibleMasses.length}
	<ul class="space-y-4">
		{#each visibleMasses as mass (mass.id)}
			<MassCard {mass} showActions={true} />
		{/each}

		<div style="height: {phantomHeight}px; width: 100%"></div>
	</ul>
{:else}
	<p class="text-gray-500 italic">No masses found.</p>
{/if}
