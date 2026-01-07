import { writable } from 'svelte/store';

export type MassUnit = 'kg' | 'lbs';

export const selectedUnit = writable<MassUnit>('kg');

export const UNIT_LABELS = {
    kg: 'Kg',
    lbs: 'Lbs'
};

export function convertFromKg(kgValue: number, unit: MassUnit): number {
    if (unit === 'kg') return kgValue;
    if (unit === 'lbs') return kgValue * 2.20462;
    return kgValue;
}

export function convertToKg(value: number, unit: MassUnit): number {
    if (unit === 'kg') return value;
    if (unit === 'lbs') return value / 2.20462;
    return value;
}

export function formatMass(value: number, unit: MassUnit): string {
    const converted = convertFromKg(value, unit);
    return converted.toLocaleString(undefined, { 
        minimumFractionDigits: 1, 
        maximumFractionDigits: 2 
    });
}
