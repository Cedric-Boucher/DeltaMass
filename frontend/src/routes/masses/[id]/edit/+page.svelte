<script lang="ts">
    import { onMount } from 'svelte';
    import { page } from '$app/state';
    import { getMass, updateMass } from '$lib/api';
    import { goto } from '$app/navigation';
    import MassForm from '$lib/components/MassForm.svelte';
    import type { NewMass, Mass } from '$lib/types';

    let mass: Mass | null = null;
    const id = page.params.id;
    const redirectTo = page.url.searchParams.get('redirectTo') ?? '/masses';

    onMount(async () => {
        if (id) {
            mass = await getMass(id);
        } else {
            goto(redirectTo);
        }
    });

    async function handleUpdate(data: NewMass) {
        if (id) {
            await updateMass(id, data);
        }
        goto(redirectTo);
    }

    function cancel() {
        goto(redirectTo);
    }
</script>

{#if mass}
    <h1 class="text-2xl font-bold mb-4">Edit Mass</h1>
    <MassForm initial={mass} onSubmit={handleUpdate} onCancel={cancel} submitLabel="Save Changes" showCancel={true} />
{:else}
    <p>Loading...</p>
{/if}
