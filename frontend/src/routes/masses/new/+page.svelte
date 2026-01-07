<script lang="ts">
    import MassForm from '$lib/components/MassForm.svelte';
    import { createMass } from '$lib/api';
    import { goto } from '$app/navigation';
	import type { NewMass } from '$lib/types';

    async function handleCreate(payload: NewMass) {
        await createMass(payload);
        goto('/masses');
    }

    function cancel() {
        goto('/masses');
    }
</script>

<h1 class="text-2xl font-bold mb-4">New Mass</h1>
<MassForm onSubmit={handleCreate} onCancel={cancel} submitLabel="Create" showCancel={true} />
