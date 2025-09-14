export function resetObject<T extends Record<string, unknown>>(obj: T) {
    for (const key in obj) {
        const value = obj[key];

        if (Array.isArray(value)) {
            obj[key] = [] as T[typeof key];
        } else if (isPlainObject(value)) {
            resetObject(value);
        } else {
            obj[key] = undefined as T[typeof key];
        }
    }
}

export function formatNumber(num: string | number) {
    const parts = num.toString().split(".");
    parts[0] = parts[0].replace(/\B(?=(\d{3})+(?!\d))/g, ",");
    return parts.join(".");
}

function isPlainObject(obj: unknown): obj is Record<string, any> {
    return (
        typeof obj === "object" &&
        obj !== null &&
        Object.getPrototypeOf(obj) === Object.prototype
    );
}

export function toReadableText(str: string) {
    return str
        .replace(/_/g, " ")
        .replace(/\b\w/g, (l) => l.toUpperCase());
}

export async function wait(seconds: number) {
    return new Promise((resolve) => {
        setTimeout(resolve, seconds * 1000);
    });
}
