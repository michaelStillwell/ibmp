import axios from "axios";
import Recipe, { NewRecipe } from "../models/Recipe";
import api from "../api/api";

export default {
    async getRecipes(): Promise<Recipe[]> {
        const response = await api.get("/recipes");
        return response.data as Recipe[];
    },

    async getRecipe(id: number): Promise<Recipe> {
        const response = await api.get(`/recipe/${id}`);
        return response.data as Recipe;
    },

    async createNewRecipe(recipe: NewRecipe): Promise<Recipe> {
        // TODO: see what comes back when creating a duplicate recipe
        const response = await api.post("/recipes", recipe);
        return response.data as Recipe;
    },

    async updateRecipe(recipe: Recipe): Promise<boolean> {
        const response = await api.put(`/recipe/${recipe.id}`, recipe);
        return response.status === 200;
    },

    async deleteRecipe(id: number): Promise<boolean> {
        const response = await api.delete(`/recipe/${id}`);
        return response.status === 200;
    },
};
