import { redirect } from '@sveltejs/kit';
import { load_user } from '$lib/api';

export async function load({ fetch }) {
	const { user } = await load_user(fetch);
	if (user) {
		throw redirect(302, '/masses');
	} else {
		throw redirect(302, '/login');
	}
}
