import http from '../http-common';
import CreateIngredientModel from '../models/Ingredient/CreateIngredientModel';
import ResponseModel from '../models/ResponseModel';

class IngredientService {
	createIngredient(createIngredientRequest: CreateIngredientModel): Promise<ResponseModel> {
		return http.post("/ingredient", createIngredientRequest)
	}
}

export default new IngredientService();