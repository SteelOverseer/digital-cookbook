<template>
  <v-container id="create-recipe-form">
    <v-row no-gutters>
      <h2>{{ props.isEdit ? 'Edit' : 'Create' }} Recipe</h2>
    </v-row>
    <v-row>
      <v-col>
        <form>
          <v-text-field
            v-model="state.name"
            label="Name"
          ></v-text-field>
          <v-select
            label="Category"
            :items="props.categories"
            item-title="name"
            item-value="id"
            v-model="state.category"
          ></v-select>
          <v-row>
            <v-col>
              <div id="ingredients">
                <div id="ingredients-header">
                  <h3>Ingredients</h3>
                  <v-btn variant="tonal" @click="state.ingredients.push(new IngredientModel())">New Ingredient</v-btn>
                </div>
                <div v-for="(ingredient, index) in state.ingredients" class="ingredient-row">
                  <v-text-field
                    v-model="state.ingredients[index].ingredient_text"
                    clearable 
                    label="Ingredient"
                  ></v-text-field>
                </div>
              </div>
            </v-col>
            <v-col>
              <draggable 
                v-model="state.instructions"
                id="instructions-list"
                item-key="element"
              >
                <template #item="{element, index}">
                  <div class="drag-item">
                    <v-text-field
                      v-model="state.instructions[index].instruction_text"
                      label="Instruction"
                    ></v-text-field>
                  </div>
                </template>
                <template #header>
                  <div id="instructions-header">
                    <h3>Instructions</h3>
                    <v-btn
                      variant="tonal"
                      id="add-instruction-button" 
                      @click="state.instructions.push(new InstructionModel())"
                    >
                      New Instruction
                    </v-btn>
                  </div>
                </template>
              </draggable>
            </v-col>
          </v-row>
          <v-textarea
            v-model="state.notes"
            label="Notes"
          ></v-textarea>
          <div id="new-recipe-footer">
            <v-btn
              id="submit-button"
              class="me-4"
              @click="submit"
            >
              Submit
            </v-btn>
            <v-btn
              id="clear-button"
              v-if="!props.isEdit"
              @click="clear"
            >
              Clear
            </v-btn>
            <v-btn
              id="cancel-button"
              v-if="props.isEdit"
              @click="$emit('cancelEdit')"
            >
              Cancel
            </v-btn>
          </div>
        </form>
      </v-col>
    </v-row>
  </v-container>
  
</template>

<script setup lang="ts">
import { reactive } from 'vue';
import CreateRecipeModel from '../models/Recipe/CreateRecipeModel';
import RecipeService from '../services/RecipeService';
import CreateIngredientModel from '../models/Ingredient/CreateIngredientModel';
import RecipeModel from '../models/Recipe/RecipeModel';
import IngredientService from '../services/IngredientService';
import draggable from 'vuedraggable'
import CreateInstructionModel from '../models/Instruction/CreateInstructionModel'
import InstructionService from '../services/InstructionService';
import CategoryModel from '../models/Category/CategoryModel';
import IngredientModel from '../models/Ingredient/IngredientModel';
import InstructionModel from '../models/Instruction/InstructionModel';

// const props = defineProps(['categories', 'recipe', 'isEdit'])
const props = defineProps<{
  categories: CategoryModel,
  recipe: RecipeModel,
  isEdit: boolean
}>()
const emit = defineEmits(['saved', 'cancelEdit'])

const state = reactive({
  name: props.recipe.name,
  notes: props.recipe.notes,
  ingredients: props.recipe.ingredients,//.map((ingredient) => { return ingredient.ingredient_text} ),
  instructions: props.recipe.instructions,//.map((instruction) => { return instruction.instruction_text }),
  category: props.recipe.category_id,
});

const clear = () => {
  state.name = ""
  state.notes = ""
  state.ingredients = []
  state.instructions = []
  state.category = ""
}

const submit = async () => {
  try {
    let recipe: RecipeModel

    if(props.isEdit) {
      const request:CreateRecipeModel =  {
        name: state.name,
        category_id: state.category,
        notes: state.notes,
      }

      let resp = await RecipeService.editRecipe(request, props.recipe.id);
      recipe = resp.data;

      Promise.all(state.ingredients.map(async ingredient => {
        if(ingredient != null && ingredient.ingredient_text != "" && ingredient.ingredient_text != " ") {
          let updateIngredientReq:IngredientModel = {
            id: ingredient.id,
            recipe_id: recipe.id,
            ingredient_text: ingredient.ingredient_text
          }
          
          await IngredientService.updateIngredient(updateIngredientReq);
        }
      }));
  
      Promise.all(state.instructions.map(async (instruction, index) => {
        if(instruction != null && instruction.instruction_text != "" && instruction.instruction_text != " ") {
          let updateInstructionReq:InstructionModel = {
            id: instruction.id,
            instruction_text: instruction.instruction_text,
            recipe_id: recipe.id,
            step_order: index
          }
          await InstructionService.updateInstruction(updateInstructionReq)
        }
      }))
    } else {
      const request:CreateRecipeModel =  {
        name: state.name,
        category_id: state.category,
        notes: state.notes,
      }

      let resp = await RecipeService.createRecipe(request);
      recipe = resp.data
      
      Promise.all(state.ingredients.map(async ingredient => {
        if(ingredient != null && ingredient.ingredient_text != "" && ingredient.ingredient_text != " ") {
          let createIngredientReq:CreateIngredientModel = {
            recipe_id: recipe.id,
            ingredient_text: ingredient.ingredient_text
          }
          
          await IngredientService.createIngredient(createIngredientReq);
        }
      }));
  
      Promise.all(state.instructions.map(async (instruction, index) => {
        if(instruction != null && instruction.instruction_text != "" && instruction.instruction_text != " ") {
          let createInstructionReq:CreateInstructionModel = {
            instruction_text: instruction.instruction_text,
            recipe_id: recipe.id,
            step_order: index
          }
          await InstructionService.createInstruction(createInstructionReq)
        }
      }))

    }
    
    emit("saved", recipe);

  } catch (error) {
      console.log("ERROR", error.response.data)
      throw(new Error(error.response.data))
  }
  
}
</script>

<style scoped>
#create-recipe-form {
  background-color: #f6eee3; 
}
.ingredient-row, .drag-item {
  display: flex;
  flex-direction: row;
}

#instructions-list {
  
}

#instructions-header, #ingredients-header {
  display: flex;
  flex-direction: row;
  justify-content: space-between;
  margin-bottom: 5px;
}

#new-recipe-footer {
  display: flex;
  flex-direction: row;
  justify-content: flex-end;
}

#submit-button {
  background-color: lightgreen;
}

#clear-button, #cancel-button {
  background-color: lightcoral;
}

#add-instruction-button {

}
</style>