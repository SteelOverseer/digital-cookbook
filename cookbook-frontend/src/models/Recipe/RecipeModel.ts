import IngredientModel from "../Ingredient/IngredientModel";
import InstructionModel from "../Instruction/InstructionModel";

export default interface RecipeModel {
    id: string,
    category_id: string,
    name: string,
    notes: string, 
    ingredients: Array<IngredientModel>,
    instructions: Array<InstructionModel>
}