import { useState } from "react";
import { Logo } from "./assets/assets";
import Recipe, { NewRecipe } from "./models/Recipe";
import recipeService from "./services/recipe.service";
import useRecipeList from "./hooks/useRecipeList";
import { Inventory } from "./features/inventory/Inventory";
import { Listbox, Transition } from "@headlessui/react";


const App = () => {
  const [selected, setSelected] = useState(people[3]);
  return (
    <div className="App">
      <Listbox value={selected} onChange={setSelected}>
        <Listbox.Label className="block text-sm font-medium text-gray-700">
          Assigned to
        </Listbox.Label>
        <div className="mt-1 relative">
          <Listbox.Button>
            <span className="flex items-center">
              
            </span>
          </Listbox.Button>
        </div>
      </Listbox>
    </div>
  );
};

export default App;
/*
function App() {
    const [recipeList, setRecipeList] = useState<Recipe[]>([]);
    // const { recipeList, setRecipeList } = useRecipeList();
    const [enabled, setEnabled] = useState<boolean>(false);

    return (
        <div className="App">
            <header className="App-header">
              <Switch
                checked={enabled}
                onChange={setEnabled}
              >
                <span>User settings</span>
              </Switch>
              <h1 className="text-3x1 font-bold underline">
                Hello world!
              </h1>
                 <Inventory />
                <img src={Logo} className="App-logo" alt="logo" />
                <button
                    onClick={async () => {
                        const result = await recipeService.createNewRecipe({
                            name: "Recipe 1",
                            description: "Recipe 1 description",
                            directions: "Don't die",
                        } as NewRecipe);
                        if (result) {
                            setRecipeList([...recipeList, result]);
                        }
                    }}
                >
                    Send Recipe
                </button>
                <button
                    onClick={async () =>
                        setRecipeList(await recipeService.getRecipes())
                    }
                >
                    Load recipes
                </button>
                {recipeList && recipeList.length > 0 && (
                    <ul>
                        {recipeList.map((r, i) => (
                            <li key={i}>{r.name}</li>
                        ))}
                    </ul>
                )}
            </header>
        </div>
    );
}
*/
