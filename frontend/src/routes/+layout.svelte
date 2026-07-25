<script lang="ts">
	import { auth } from '$lib/stores/auth.svelte';
	import { selectedUnit, type MassUnit } from '$lib/stores/units';
	import { goto } from '$app/navigation';
	import '../app.css';
	import { onMount, type Snippet } from 'svelte';
	import { check_login, logout } from '$lib/api';
	import { page } from '$app/state';
	import { exportUserDataToFile, importUserDataFromFile } from '$lib/utils';
	import { resolve } from '$app/paths';
	import AsyncButton from '$lib/components/AsyncButton.svelte';

	let { children }: { children: Snippet } = $props();

	const currentPath = $derived(page.url.pathname);

	let darkMode = $state(false);

	onMount(check_login);

	onMount(() => {
		const storedTheme = localStorage.getItem('theme');
		if (storedTheme === 'dark') {
			darkMode = true;
			document.documentElement.classList.add('dark');
		} else {
			darkMode = false;
			document.documentElement.classList.remove('dark');
		}

		const storedUnit = localStorage.getItem('massUnit');
		if (storedUnit === 'kg' || storedUnit === 'lbs') {
			selectedUnit.set(storedUnit as MassUnit);
		}
	});

	function toggleDarkMode() {
		darkMode = !darkMode;
		document.documentElement.classList.toggle('dark', darkMode);
		localStorage.setItem('theme', darkMode ? 'dark' : 'light');
	}

	function handleUnitChange(event: Event) {
		const select = event.target as HTMLSelectElement;
		const newUnit = select.value as MassUnit;

		selectedUnit.set(newUnit);
		localStorage.setItem('massUnit', newUnit);
	}

	function navButtonClasses(pathPrefix: string): string {
		const base = 'px-3 py-2 rounded font-medium';
		const active = 'bg-blue-100 dark:bg-gray-700 text-blue-900 dark:text-white';
		const inactive = 'hover:bg-blue-100 dark:hover:bg-gray-700 text-blue-700 dark:text-blue-300';

		return `${base} ${currentPath.startsWith(pathPrefix) ? active : inactive}`;
	}
</script>

<svelte:head>
	<title>DeltaMass</title>
	<link rel="icon" href={darkMode ? '/favicon-dark.svg' : '/favicon-light.svg'} type="image/svg" />
</svelte:head>

<nav class="bg-white dark:bg-gray-800 shadow sticky top-0 z-50">
	<div class="max-w-4xl mx-auto px-4 py-3 flex justify-between items-center">
		{#if auth.isLoggedIn}
			<div class="flex gap-4">
				<AsyncButton
					action={async () => await goto(resolve('/masses'))}
					class={navButtonClasses('/masses')}
				>
					Masses
				</AsyncButton>
				<AsyncButton
					action={async () => await goto(resolve('/chart'))}
					class={navButtonClasses('/chart')}
				>
					Mass Chart
				</AsyncButton>
			</div>
		{/if}

		<div class="flex gap-4 items-center">
			{#if auth.isLoggedIn}
				<AsyncButton
					action={logout}
					class="px-3 py-2 rounded hover:bg-blue-100 dark:hover:bg-gray-700 text-red-500 dark:text-red-500 font-medium"
				>
					Logout
				</AsyncButton>

				<AsyncButton
					action={importUserDataFromFile}
					class="px-3 py-2 rounded hover:bg-blue-100 dark:hover:bg-gray-700 text-blue-700 dark:text-blue-300 font-medium"
					>Import</AsyncButton
				>
				<AsyncButton
					action={exportUserDataToFile}
					class="px-3 py-2 rounded hover:bg-blue-100 dark:hover:bg-gray-700 text-blue-700 dark:text-blue-300 font-medium"
					>Export</AsyncButton
				>
			{:else}
				<AsyncButton
					class="px-3 py-2 rounded hover:bg-blue-100 dark:hover:bg-gray-700 text-blue-700 dark:text-blue-300 font-medium"
					action={async () => await goto(resolve('/login'))}
				>
					Login
				</AsyncButton>
				<AsyncButton
					class="px-3 py-2 rounded hover:bg-blue-100 dark:hover:bg-gray-700 text-blue-700 dark:text-blue-300 font-medium"
					action={async () => await goto(resolve('/signup'))}
				>
					Sign Up
				</AsyncButton>
			{/if}

			<div class="h-6 w-px bg-gray-300 dark:bg-gray-600 mx-1"></div>

			<div class="relative">
				<select
					value={$selectedUnit}
					onchange={handleUnitChange}
					class="appearance-none px-3 py-2 pr-8 rounded text-sm bg-gray-200 dark:bg-gray-700 hover:bg-gray-300 dark:hover:bg-gray-600 text-gray-900 dark:text-gray-100 border-none cursor-pointer focus:ring-2 focus:ring-blue-500"
				>
					<option value="kg">Kg</option>
					<option value="lbs">Lbs</option>
				</select>
				<div
					class="pointer-events-none absolute inset-y-0 right-0 flex items-center px-2 text-gray-700 dark:text-gray-300"
				>
					<svg class="fill-current h-4 w-4" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20"
						><path
							d="M9.293 12.95l.707.707L15.657 8l-1.414-1.414L10 10.828 5.757 6.586 4.343 8z"
						/></svg
					>
				</div>
			</div>

			<button
				onclick={toggleDarkMode}
				class="px-3 py-2 rounded text-sm bg-gray-200 dark:bg-gray-700 hover:bg-gray-300 dark:hover:bg-gray-600 text-gray-900 dark:text-gray-100"
			>
				{darkMode ? '🌙' : '☀️'}
			</button>
		</div>
	</div>
</nav>

<main class="max-w-4xl mx-auto px-4 py-6">
	{@render children()}
</main>
