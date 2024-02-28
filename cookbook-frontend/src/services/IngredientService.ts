import http from '../http-common';
import CreateIngredientModel from '../models/Ingredient/CreateIngredientModel';
import ResponseModel from '../models/ResponseModel';

class IngredientService {
	createIngredient(createIngredientRequest: CreateIngredientModel): Promise<ResponseModel> {
		return http.post("/ingredient", createIngredientRequest)
	}

	getRecipeIngredients(recipe_id:string): Promise<ResponseModel> {
		return http.get(`/ingredient/${recipe_id}`)
	}
}

export default new IngredientService();