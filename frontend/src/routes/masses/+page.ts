import { redirect } from '@sveltejs/kit';
import { getMasses, load_user } from '$lib/api';
import type { PageLoad } from './$types';

export const load: PageLoad = async () => {
    const { user } = await load_user();

    if (!user) {
        throw redirect(302, '/login');
    }

    const masses = await getMasses();
    return { masses };
};
