export default class CreateRecipeModel {
    name: string
    category_id: string
    notes: string

    constructor() {
        this.category_id = "",
        this.name = "",
        this.notes = ""
    }
}