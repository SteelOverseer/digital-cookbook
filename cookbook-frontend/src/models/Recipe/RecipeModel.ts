import IngredientModel from "../Ingredient/IngredientModel";

export default interface RecipeModel {
    id: string,
    category_id: string,
    name: string,
    notes: string, 
    ingredients: Array<IngredientModel>
}