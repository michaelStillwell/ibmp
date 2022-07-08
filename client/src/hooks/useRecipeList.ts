import { useState, useEffect } from "react";
import api from "../api/api";
import Recipe from "../models/Recipe";

const useRecipeList = () => {
    const [recipeList, setRecipeList] = useState<Recipe[]>([]);

    useEffect(() => {
        const fetch = async () => {
            try {
                const response = await api.get("/recipes");
                setRecipeList([...recipeList, response.data]);
            } catch (e) {
                console.error(e);
            }
        };
        fetch();
    });

    return { recipeList, setRecipeList };
};

export default useRecipeList;
