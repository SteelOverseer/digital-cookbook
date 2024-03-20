export default class CreateRecipeModel {
    name?: string
    category_id?: string
    notes?: string
    is_favorite: boolean

    constructor() {
        this.category_id = "",
        this.name = "",
        this.notes = ""
        this.is_favorite = false
    }
}