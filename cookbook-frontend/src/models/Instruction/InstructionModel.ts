export default class InstructionModel {
    id: string
    instruction_text: string
    recipe_id: string
    step_order: number

    constructor() {
        this.id = "",
        this.instruction_text = "",
        this.recipe_id = "",
        this.step_order = 0
    }
}