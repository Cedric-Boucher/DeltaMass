import { goto } from "$app/navigation";
import { auth } from "./stores/auth";
import type { Mass, NewMass, NewUser, User } from "./types";

const API_BASE = '/api';

export async function getMasses(): Promise<Mass[]> {
    const res = await fetch(`${API_BASE}/masses`, {
        credentials: 'include'
    });
    if (!res.ok) throw new Error('Failed to fetch masses');
    return await res.json();
}

export async function createMass(payload: NewMass): Promise<Mass> {
    // Convert timestamp to ISO format (UTC) if present
    if (payload.created_at) {
        payload.created_at = new Date(payload.created_at).toISOString();
    }

    const res = await fetch(`${API_BASE}/masses`, {
        method: 'POST',
        headers: { 'content-type': 'application/json' },
        body: JSON.stringify(payload),
        credentials: 'include',
    });

    if (!res.ok) {
        throw new Error('Failed to create mass');
    }

    return await res.json();
}

export async function login(payload: NewUser): Promise<void> {
    const res = await fetch(`${API_BASE}/login`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(payload),
        credentials: 'include',
    });

    if (res.ok) {
        auth.set({ isLoggedIn: true });
    } else {
        throw new Error('Invalid credentials');
    }
}

export async function signup(payload: NewUser): Promise<void> {
    const res = await fetch(`${API_BASE}/signup`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(payload),
        credentials: 'include',
    });


    if (res.ok) {
        auth.set({ isLoggedIn: true });
    } else {
        throw new Error((await res.text()).valueOf());
    }
}

export async function logout(): Promise<void> {
    await fetch(`${API_BASE}/logout`, {
        method: 'POST',
        credentials: 'include',
    });
    auth.set({ isLoggedIn: false });
    goto('/login');
}

export async function check_login(): Promise<void> {
    const res = await fetch(`${API_BASE}/me`, {
        credentials: 'include'
    });
    auth.set({ isLoggedIn: res.ok });
}

export async function load_user(): Promise<{user: User | null}> {
    const res = await fetch(`${API_BASE}/me`, {
        credentials: 'include'
    });
    if (res.ok) {
        const user = await res.json();
        return { user };
    } else {
        return { user: null };
    }
};

export async function getMass(id: string): Promise<Mass> {
    const res = await fetch(`${API_BASE}/masses/${id}`, {
        credentials: 'include'
    });
    return await res.json();
}

export async function updateMass(id: string, data: NewMass) {
    await fetch(`${API_BASE}/masses/${id}`, {
        method: 'PUT',
        headers: { 'content-type': 'application/json' },
        body: JSON.stringify(data),
        credentials: 'include'
    });
}

export async function deleteMass(id: string) {
    await fetch(`${API_BASE}/masses/${id}`, {
        method: 'DELETE',
        credentials: 'include'
    });
}

export async function uploadUserData(payload: string) {
    const data = JSON.parse(payload);

    const res = await fetch(`${API_BASE}/import`, {
        method: 'POST',
        headers: {
            'content-type': 'application/json',
        },
        body: JSON.stringify(data),
        credentials: 'include'
    });

    if (res.ok) {
        alert('Data imported successfully!');
    } else {
        const error = await res.text();
        alert(`Import failed: ${error}`);
    }
}
