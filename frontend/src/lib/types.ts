export type Mass = {
    id: number;
    mass_kg: number;
    measurement_timestamp: string;
    created_at: string;
};

export type NewMass = {
    mass_kg: number;
    measurement_timestamp?: string;
    created_at?: string;
};

export type NewUser = {
    username: string;
    password: string;
};

export type User = {
    id: string;
    username: string;
    password_hash: string;
    created_at: string;
};
