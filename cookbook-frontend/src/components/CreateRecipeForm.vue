<template>
  <v-container id="create-recipe-form">
    <v-row no-gutters>
      <h2>Create Recipe</h2>
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
            {{ props.recipe }}
            <v-col>
              <div id="ingredients">
                <div id="ingredients-header">
                  <h3>Ingredients</h3>
                  <v-btn variant="tonal" @click="state.ingredients.push('')">New Ingredient</v-btn>
                </div>
                <div v-for="(ingredient, index) in state.ingredients" class="ingredient-row">
                  <v-text-field
                    v-model="state.ingredients[index]"
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
                      v-model="state.instructions[index]"
                      clearable
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
                      @click="state.instructions.push('')"
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
              @click="clear"
            >
              Clear
            </v-btn>
          </div>
        </form>
      </v-col>
    </v-row>
  </v-container>
  
</template>

<script setup lang="ts">
import { reactive } from 'vue';
import CreateRecipeModel from '../models/CreateRecipeModel';
import RecipeService from '../services/RecipeService';
import CreateIngredientModel from '../models/Ingredient/CreateIngredientModel';
import RecipeModel from '../models/Recipe/RecipeModel';
import IngredientService from '../services/IngredientService';
import draggable from 'vuedraggable'
import CreateInstructionModel from '../models/Instruction/CreateInstructionModel'
import InstructionService from '../services/InstructionService';

const props = defineProps(['categories', 'recipe', 'isEdit'])
const emit = defineEmits(['saved'])

const state = reactive({
  name: "",
  notes: "",
  ingredients: [""],
  instructions: [""],
  category: "",    
});

const clear = () => {
  state.name = ""
  state.notes = ""
  state.ingredients = [""]
  state.instructions = [""]
  state.category = ""
}

const submit = async () => {
  const request:CreateRecipeModel =  {
    name: state.name,
    category_id: state.category,
    notes: state.notes,
  }

  try {
    let resp = await RecipeService.createRecipe(request);
    let recipe:RecipeModel = resp.data

    Promise.all(state.ingredients.map(async ingredient => {
      if(ingredient != null && ingredient != "" && ingredient != " ") {
        let createIngredientReq:CreateIngredientModel = {
          recipe_id: recipe.id,
          ingredient_text: ingredient
        }

        await IngredientService.createIngredient(createIngredientReq);
      }
    }));

    Promise.all(state.instructions.map(async (instruction, index) => {
      if(instruction != null && instruction != "" && instruction != " ") {
        let createInstructionReq:CreateInstructionModel = {
          instruction_text: instruction,
          recipe_id: recipe.id,
          step_order: index
        }
        await InstructionService.createInstruction(createInstructionReq)
      }
    }))

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

#clear-button {
  background-color: lightcoral;
}

#add-instruction-button {

}
</style>