import http from '../http-common';
import CreateIngredientModel from '../models/Ingredient/CreateIngredientModel';
import IngredientModel from '../models/Ingredient/IngredientModel';
import ResponseModel from '../models/ResponseModel';

class IngredientService {
	createIngredient(createIngredientRequest: CreateIngredientModel): Promise<ResponseModel> {
		return http.post("/ingredient", createIngredientRequest)
	}

	getRecipeIngredients(recipe_id:string): Promise<ResponseModel> {
		return http.get(`/ingredient/${recipe_id}`)
	}

	updateIngredient(updateIngredientRequest: IngredientModel): Promise<ResponseModel> {
		return http.patch(`/ingredient/${updateIngredientRequest.id}`, updateIngredientRequest)
	}
}

export default new IngredientService();