import IngredientModel from "../Ingredient/IngredientModel";
import InstructionModel from "../Instruction/InstructionModel";

export default class RecipeModel {
    id: string
    category_id: string
    name: string
    notes: string 
    ingredients: Array<IngredientModel>
    instructions: Array<InstructionModel>
    is_favorite: boolean

    constructor() {
        this.id = "",
        this.category_id = "",
        this.name = "",
        this.notes = "",
        this.ingredients = [],
        this.instructions = [],
        this.is_favorite = false
    }
}