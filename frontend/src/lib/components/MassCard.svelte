<script lang="ts">
    import type { Mass } from '$lib/types';
    import { formatTimestampLocalForDisplay } from '$lib/utils';
    import { goto } from '$app/navigation';
    import { page } from '$app/state';

    import { selectedUnit, formatMass, UNIT_LABELS } from '$lib/stores/units';

    export let mass: Mass;
    export let showActions: boolean = true;

    export let onEdit: (() => void) | null = null;
    export let onDelete: (() => void) | null = null;

    function handleEdit() {
        if (onEdit) {
            onEdit();
        } else {
            goto(`/masses/${mass.id}/edit?redirectTo=${encodeURIComponent(page.url.pathname)}`);
        }
    }

    function handleDelete() {
        if (onDelete) {
            onDelete();
        } else {
            goto(`/masses/${mass.id}/delete?redirectTo=${encodeURIComponent(page.url.pathname)}`);
        }
    }
</script>

<li class="bg-white dark:bg-gray-800 shadow rounded p-4 flex justify-between items-start gap-4">
    <div>
        <div class="text-2xl font-bold">
            {formatMass(mass.mass_kg, $selectedUnit)} {UNIT_LABELS[$selectedUnit]}
        </div>
        <div class="text-gray-500 text-sm">{formatTimestampLocalForDisplay(mass.measurement_timestamp)}</div>
    </div>

    {#if showActions}
        <div class="flex flex-col gap-2">
            <button
                type="button"
                class="bg-blue-600 text-white px-4 py-2 rounded hover:bg-blue-700"
                on:click={handleEdit}
            >
                Edit
            </button>
            <button
                type="button"
                class="bg-red-600 text-white px-4 py-2 rounded hover:bg-red-700"
                on:click={handleDelete}
            >
                Delete
            </button>
        </div>
    {/if}
</li>
