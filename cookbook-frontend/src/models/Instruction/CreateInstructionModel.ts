export default class CreateInstructionModel {
    instruction_text: string
    recipe_id: string
    step_order: number

    constructor() {
        this.instruction_text = "",
        this.recipe_id = "",
        this.step_order = 0
    }
}