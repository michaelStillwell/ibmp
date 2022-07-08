export default interface Recipe {
    id?: number;
    name?: string;
    description?: string;
    directions?: string;
}

export interface NewRecipe {
    name: string;
    description: string;
    directions: string;
    // ingredients: string[];
}
