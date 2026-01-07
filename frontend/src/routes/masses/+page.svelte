<script lang="ts">
    import { goto } from '$app/navigation';
    import type { Mass } from '$lib/types';
    import MassCard from '$lib/components/MassCard.svelte';

    export let data: { masses: Mass[] };

    const BATCH_SIZE = 100;
    const ESTIMATED_ITEM_HEIGHT = 80; 

    let limit = BATCH_SIZE;

    $: {
        data.masses;
        limit = BATCH_SIZE;
        if (typeof window !== 'undefined') window.scrollTo(0, 0);
    }

    $: if (limit < data.masses.length) {
        setTimeout(() => {
            limit += BATCH_SIZE;
        }, 0);
    }

    $: visibleMasses = data.masses.slice(0, limit);

    $: remainingCount = Math.max(0, data.masses.length - limit);
    $: phantomHeight = remainingCount * ESTIMATED_ITEM_HEIGHT;

</script>

<h1 class="text-2xl font-bold mb-4">Masses</h1>

<div class="mb-6 flex items-center gap-4">
    <button
        class="bg-blue-600 text-white px-4 py-2 rounded hover:bg-blue-700"
        on:click={() => goto('/masses/new')}
    >
        + New Mass
    </button>
</div>

{#if visibleMasses.length}
    <ul class="space-y-4">
        {#each visibleMasses as tx (tx.id)}
            <MassCard mass={tx} showActions={true} />
        {/each}

        <div style="height: {phantomHeight}px; width: 100%"></div>
    </ul>

    {#if limit < data.masses.length}
        <p class="text-center text-xs text-gray-400 mt-2">Loading rest of data...</p>
    {:else}
        <p class="text-center text-xs text-gray-400 mt-2 mb-8">
            Showing all {data.masses.length} masses
        </p>
    {/if}
{:else}
    <p class="text-gray-500 italic">No masses to show.</p>
{/if}
