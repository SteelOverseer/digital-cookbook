import http from '../http-common';
import CreateIngredientModel from '../models/Ingredient/CreateIngredientModel';

class IngredientService {
	createIngredient(createIngredientRequest: CreateIngredientModel): Promise<any> {
		return http.post("/ingredient", createIngredientRequest)
	}
}

export default new IngredientService();