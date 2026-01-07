<script lang="ts">
    import { page } from '$app/state';
    import { getMass, deleteMass } from '$lib/api';
    import { goto } from '$app/navigation';
    import { onMount } from 'svelte';
    import type { Mass } from '$lib/types';
	import MassCard from '$lib/components/MassCard.svelte';

    let mass: Mass | null = null;
    let error = '';
    let loading = true;

    const id = page.params.id;
    const redirectTo = page.url.searchParams.get('redirectTo') ?? '/masses';

    onMount(async () => {
        if (!id) {
            goto(redirectTo);
        } else {
            try {
                mass = await getMass(id);
            } catch (e) {
                error = 'Failed to load mass.';
                console.error(e);
            } finally {
                loading = false;
            }
        }
    });

    async function confirmDelete() {
        try {
            if (id) {
                await deleteMass(id);
            }
            goto(redirectTo);
        } catch (e) {
            error = 'Failed to delete mass.';
            console.error(e);
        }
    }

    function cancel() {
        goto(redirectTo);
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
        <button
            on:click={confirmDelete}
            class="bg-red-600 text-white px-4 py-2 rounded hover:bg-red-700"
        >
            Yes, Delete
        </button>
        <button
            on:click={cancel}
            class="bg-gray-300 dark:bg-gray-700 text-black dark:text-white px-4 py-2 rounded hover:bg-gray-400 dark:hover:bg-gray-600"
        >
            Cancel
        </button>
    </div>
{:else}
    <p class="text-gray-500 italic">Mass not found.</p>
{/if}
