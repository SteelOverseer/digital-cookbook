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
}

export default new RecipeService();