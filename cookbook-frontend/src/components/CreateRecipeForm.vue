<template>
  <v-container>
    <v-row no-gutters>
      <v-col>Create Recipe</v-col>
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
          <div id="ingredients">
            <div v-for="(ingredient, index) in state.ingredients" class="ingredient-row">
              <v-text-field
                v-model="state.ingredients[index]"
                clearable 
                label="Ingredient"
              ></v-text-field>
              <v-btn variant="tonal" @click="state.ingredients.push('')">+</v-btn>
            </div>
          </div>
          <draggable 
            v-model="state.instructions"
            id="instructions-list"
            item-key="element">
            <template #item="{element, index}">
              <div class="drag-item">
                <v-text-field
                  v-model="state.instructions[index]"
                  clearable
                ></v-text-field>
              </div>
            </template>
            <template #header>
              <div id="instructions-header">
                <h3>Instructions</h3>
                <v-btn @click="state.instructions.push('')">Add Instruction</v-btn>
              </div>
            </template>
          </draggable>
          <v-textarea
            v-model="state.notes"
            label="Notes"
          ></v-textarea>
          <v-btn
            class="me-4"
            @click="submit"
          >
            submit
          </v-btn>
          <v-btn
            @click="clear"
          >
            clear
          </v-btn>
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

const props = defineProps(['categories'])
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
        console.log('createinstructionreq', createInstructionReq)
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
.ingredient-row {
  display: flex;
  flex-direction: row;
}

#instructions-list {
  
}

#instructions-header {
  display: flex;
  flex-direction: column;
}

</style>