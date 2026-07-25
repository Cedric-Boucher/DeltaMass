import { createQuery, createMutation, useQueryClient } from '@tanstack/svelte-query';
import { getMasses, createMass, updateMass, deleteMass } from '$lib/api';
import type { Mass, NewMass } from '$lib/types';

export interface AppData {
	masses: Mass[];
}

export function useAppData() {
	return createQuery<AppData, Error>(() => ({
		queryKey: ['appData'],
		queryFn: async (): Promise<AppData> => {
			const [masses] = await Promise.all([getMasses()]);

			return { masses };
		}
	}));
}

// --- Mass Mutations ---

export function useCreateMass() {
	const queryClient = useQueryClient();

	return createMutation(() => ({
		mutationFn: (payload: NewMass) => createMass(payload),
		onSuccess: () => {
			queryClient.invalidateQueries({ queryKey: ['appData'] });
		}
	}));
}

export function useUpdateMass() {
	const queryClient = useQueryClient();

	return createMutation(() => ({
		// mutationFn only takes one variable, so we pass an object containing both id and data
		mutationFn: ({ id, data }: { id: string; data: NewMass }) => updateMass(id, data),
		onSuccess: () => {
			queryClient.invalidateQueries({ queryKey: ['appData'] });
		}
	}));
}

export function useDeleteMass() {
	const queryClient = useQueryClient();

	return createMutation(() => ({
		mutationFn: (id: string) => deleteMass(id),
		onSuccess: () => {
			queryClient.invalidateQueries({ queryKey: ['appData'] });
		}
	}));
}
