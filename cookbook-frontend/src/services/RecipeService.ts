import http from '../http-common';
import CreateRecipeModel from '../models/Recipe/CreateRecipeModel';

class RecipeService {
	getAll(): Promise<any> {
		return http.get("/recipes");
	}

	createRecipe(createRecipeRequest: CreateRecipeModel): Promise<any> {
		return http.post("/recipe", createRecipeRequest)
	}
}

export default new RecipeService();