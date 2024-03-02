import http from '../http-common';
import CreateInstructionMode from '../models/Instruction/CreateInstructionModel';
import ResponseModel from '../models/ResponseModel';

class InstructionService {
	createInstruction(createInstructionRequest: CreateInstructionMode): Promise<ResponseModel> {
		return http.post("/instruction", createInstructionRequest)
	}

	getRecipeInstructions(recipe_id:string): Promise<ResponseModel> {
		return http.get(`/instructions/recipe/${recipe_id}`)
	}
}

export default new InstructionService();