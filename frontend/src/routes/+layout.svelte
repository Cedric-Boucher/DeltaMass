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
	import type { Pathname } from '$app/types';
	import AsyncButton from '$lib/components/AsyncButton.svelte';
	import { QueryClient, QueryClientProvider } from '@tanstack/svelte-query';

	const queryClient = new QueryClient({
		defaultOptions: {
			queries: {
				refetchInterval: 15000, // Poll every 15 seconds
				refetchIntervalInBackground: false, // Pause when tab is hidden
				refetchOnWindowFocus: true, // Fetch immediately when tab regains focus
				staleTime: 10000 // Consider data stale after 10 seconds
			}
		}
	});

	let { children }: { children: Snippet } = $props();

	const currentPath = $derived(page.url.pathname);

	let darkMode = $state(false);
	let isMobileMenuOpen = $state(false);
	let isSettingsOpen = $state(false);

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
		const base = 'px-3 py-2 rounded font-medium text-left md:text-center transition-colors';
		const active = 'bg-blue-100 dark:bg-gray-700 text-blue-900 dark:text-white';
		const inactive = 'hover:bg-blue-100 dark:hover:bg-gray-700 text-blue-700 dark:text-blue-300';

		return `${base} ${currentPath.startsWith(pathPrefix) ? active : inactive}`;
	}

	async function handleNav(path: string) {
		isMobileMenuOpen = false;
		await goto(resolve(path as Pathname));
	}

	async function handleDropdownAction(action: () => void | Promise<void>) {
		await action();
		isSettingsOpen = false;
	}
</script>

<svelte:head>
	<title>DeltaMass</title>
	<link rel="icon" href={darkMode ? '/favicon-dark.svg' : '/favicon-light.svg'} type="image/svg" />
</svelte:head>

<QueryClientProvider client={queryClient}>
	<nav class="bg-white dark:bg-gray-800 shadow sticky top-0 z-50">
		<div class="max-w-4xl mx-auto px-4">
			<div class="flex justify-between items-center py-3">
				<div class="flex items-center gap-4">
					{#if auth.isLoggedIn}
						<button
							onclick={() => (isMobileMenuOpen = !isMobileMenuOpen)}
							class="md:hidden p-2 -ml-2 rounded hover:bg-gray-100 dark:hover:bg-gray-700 text-gray-600 dark:text-gray-300"
							aria-label="Toggle Menu"
						>
							<svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								{#if isMobileMenuOpen}
									<path
										stroke-linecap="round"
										stroke-linejoin="round"
										stroke-width="2"
										d="M6 18L18 6M6 6l12 12"
									/>
								{:else}
									<path
										stroke-linecap="round"
										stroke-linejoin="round"
										stroke-width="2"
										d="M4 6h16M4 12h16M4 18h16"
									/>
								{/if}
							</svg>
						</button>

						<span class="font-bold text-gray-800 dark:text-gray-200 md:hidden"> DeltaMass </span>

						<div class="hidden md:flex gap-2">
							<AsyncButton action={() => handleNav('/masses')} class={navButtonClasses('/masses')}>
								Masses
							</AsyncButton>
							<AsyncButton action={() => handleNav('/chart')} class={navButtonClasses('/chart')}>
								Mass Chart
							</AsyncButton>
						</div>
					{:else}
						<span class="font-bold text-gray-800 dark:text-gray-200">DeltaMass</span>
					{/if}
				</div>

				<div class="flex items-center gap-2 sm:gap-3">
					{#if auth.isLoggedIn}
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
								<svg
									class="fill-current h-4 w-4"
									xmlns="http://www.w3.org/2000/svg"
									viewBox="0 0 20 20"
								>
									<path
										d="M9.293 12.95l.707.707L15.657 8l-1.414-1.414L10 10.828 5.757 6.586 4.343 8z"
									/>
								</svg>
							</div>
						</div>

						<div class="relative">
							<button
								onclick={() => (isSettingsOpen = !isSettingsOpen)}
								class="px-3 py-2 rounded flex items-center gap-1 font-medium hover:bg-gray-100 dark:hover:bg-gray-700 text-gray-700 dark:text-gray-200 transition-colors"
							>
								Settings
								<svg
									class="w-4 h-4 transition-transform {isSettingsOpen ? 'rotate-180' : ''}"
									fill="none"
									stroke="currentColor"
									viewBox="0 0 24 24"
								>
									<path
										stroke-linecap="round"
										stroke-linejoin="round"
										stroke-width="2"
										d="M19 9l-7 7-7-7"
									/>
								</svg>
							</button>

							{#if isSettingsOpen}
								<div
									class="fixed inset-0 z-40"
									onclick={() => (isSettingsOpen = false)}
									onkeydown={(e) => e.key === 'Escape' && (isSettingsOpen = false)}
									role="button"
									tabindex="0"
									aria-label="Close Settings"
								></div>

								<div
									class="absolute right-0 mt-1 w-48 bg-white dark:bg-gray-800 rounded shadow-lg border border-gray-100 dark:border-gray-700 z-50 py-1 overflow-hidden"
								>
									<AsyncButton
										action={() => handleDropdownAction(importUserDataFromFile)}
										class="w-full text-left px-4 py-2 text-sm text-gray-700 dark:text-gray-200 hover:bg-blue-50 dark:hover:bg-gray-700 transition-colors"
									>
										Import Data
									</AsyncButton>
									<AsyncButton
										action={() => handleDropdownAction(exportUserDataToFile)}
										class="w-full text-left px-4 py-2 text-sm text-gray-700 dark:text-gray-200 hover:bg-blue-50 dark:hover:bg-gray-700 transition-colors"
									>
										Export All Data
									</AsyncButton>
									<button
										onclick={() => handleDropdownAction(toggleDarkMode)}
										class="w-full text-left px-4 py-2 text-sm text-gray-700 dark:text-gray-200 hover:bg-blue-50 dark:hover:bg-gray-700 transition-colors border-t border-gray-100 dark:border-gray-700"
									>
										{darkMode ? 'Switch to Light Mode' : 'Switch to Dark Mode'}
									</button>
									<AsyncButton
										action={() => handleDropdownAction(logout)}
										class="w-full text-left px-4 py-2 text-sm font-medium text-red-600 dark:text-red-400 hover:bg-red-50 dark:hover:bg-red-900/20 transition-colors border-t border-gray-100 dark:border-gray-700"
									>
										Logout
									</AsyncButton>
								</div>
							{/if}
						</div>
					{:else}
						<AsyncButton
							class="px-3 py-2 rounded text-sm sm:text-base hover:bg-blue-100 dark:hover:bg-gray-700 text-blue-700 dark:text-blue-300 font-medium transition-colors"
							action={() => handleNav('/login')}
						>
							Login
						</AsyncButton>
						<AsyncButton
							class="px-3 py-2 rounded text-sm sm:text-base hover:bg-blue-100 dark:hover:bg-gray-700 text-blue-700 dark:text-blue-300 font-medium transition-colors"
							action={() => handleNav('/signup')}
						>
							Sign Up
						</AsyncButton>
						<button
							onclick={toggleDarkMode}
							class="px-3 py-2 rounded text-sm bg-gray-200 dark:bg-gray-700 hover:bg-gray-300 dark:hover:bg-gray-600 text-gray-900 dark:text-gray-100 transition-colors"
							aria-label="Toggle Dark Mode"
						>
							{darkMode ? '🌙' : '☀️'}
						</button>
					{/if}
				</div>
			</div>

			{#if auth.isLoggedIn && isMobileMenuOpen}
				<div
					class="md:hidden flex flex-col gap-1 pb-4 pt-2 border-t border-gray-100 dark:border-gray-700"
				>
					<AsyncButton action={() => handleNav('/masses')} class={navButtonClasses('/masses')}>
						Masses
					</AsyncButton>
					<AsyncButton action={() => handleNav('/chart')} class={navButtonClasses('/chart')}>
						Mass Chart
					</AsyncButton>
				</div>
			{/if}
		</div>
	</nav>

	<main class="max-w-4xl mx-auto px-4 py-6">
		{@render children()}
	</main>
</QueryClientProvider>
