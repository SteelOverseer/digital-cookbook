import http from '../http-common';
import CreateRecipeModel from '../models/Recipe/CreateRecipeModel';
import ResponseModel from '../models/ResponseModel';

class RecipeService {
	getAll(): Promise<ResponseModel> {
		return http.get("/recipes");
	}

	createRecipe(createRecipeRequest: CreateRecipeModel): Promise<ResponseModel> {
		return http.post("/recipe", createRecipeRequest)
	}

	editRecipe(editRecipeRequest: CreateRecipeModel, recipeID: string): Promise<ResponseModel> {
		return http.patch(`/recipe/${recipeID}`, editRecipeRequest)
	}
}

export default new RecipeService();