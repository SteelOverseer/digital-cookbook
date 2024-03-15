export default class IngredientModel {
    id: string
    recipe_id: string
    ingredient_text: string

    constructor() {
        this.id = "",
        this.recipe_id = "",
        this.ingredient_text = ""
    }
}