<template>
  <v-container id="app-container">
    <v-row no-gutters id="header">
      <v-col>
		<!-- <v-toolbar style="background-color: grey; color:red">
			<v-app-bar-nav-icon></v-app-bar-nav-icon>
			<v-toolbar-title>Schultz Family Cookbook</v-toolbar-title>
		</v-toolbar> -->
        <h1>Schultz Family Cookbook</h1>
      </v-col>
    </v-row>
    <v-row no-gutters>
      <v-col cols="2">
        <v-btn variant="tonal" @click="state.showNewCategoryDialog = true">New Category</v-btn>
        <v-btn variant="tonal" @click="state.createNewRecipe = true">New Recipe</v-btn>
        <v-sheet class="pa-2 ma-2" id="categories-accordian">
          <Categories :data="state.accordianData" @select-recipe="(recipe) => onRecipeSelected(recipe)" />
        </v-sheet>
      </v-col>
      <v-col>
        <v-sheet class="pa-2 ma-2">
          Select a recipe on the left or create one to get started
          selected recipe: {{ state.selectedRecipe }}
          <CreateRecipeForm 
            v-if="state.createNewRecipe" 
            :categories="state.categories"
            @saved="(recipe) => onNewRecipeSaved(recipe)"
           />
          <Recipe v-if="!state.createNewRecipe" :recipe="state.selectedRecipe" />
        </v-sheet>
      </v-col>
    </v-row>
    <v-snackbar
      v-model="state.showToast"
      multi-line
	    location="top right"
    >
      {{ state.message }}
      <template v-slot:actions>
        <v-btn
          color="red"
          variant="text"
          @click="state.showToast = false"
        >
          Close
        </v-btn>
      </template>
    </v-snackbar>
  </v-container>
  <v-dialog
    v-model="state.showNewCategoryDialog"
    persistent
    width="300"
  >
    <v-card>
      <v-card-title>
        <span class="text-h5">Create Category</span>
      </v-card-title>
      <v-card-text>
        <v-container>
          <v-row>
            <v-col
              cols="12"
              sm="8"
              md="10"
            >
              <v-text-field
                label="Category Name*"
                required
                v-model="state.newCategoryName"
              ></v-text-field>
            </v-col>
          </v-row>
        </v-container>
        <small>*indicates required field</small>
      </v-card-text>
      <v-card-actions>
        <v-spacer></v-spacer>
        <v-btn
          color="blue-darken-1"
          variant="text"
          @click="state.showNewCategoryDialog = false"
        >
          Close
        </v-btn>
        <v-btn
          color="blue-darken-1"
          variant="text"
          @click="createCategory()"
        >
          Save
        </v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script setup lang="ts">
import Categories from './components/Categories.vue'
import CreateRecipeForm from './components/CreateRecipeForm.vue'
import { onErrorCaptured, reactive, ref } from 'vue';
import CategoryService from './services/CategoryService';
import CategoryModel from './models/Category/CategoryModel';
import RecipeService from './services/RecipeService';
import RecipeModel from './models/Recipe/RecipeModel';
import Recipe from './components/Recipe.vue'
import IngredientService from './services/IngredientService';
import InstructionService from './services/InstructionService';

const state = reactive({
  categories: [] as CategoryModel[],
  recipes: [] as RecipeModel[],
	showToast: false,
	message: "",
  accordianData: [] as any,
  loading: true,
  showNewCategoryDialog: false,
  showNewRecipeDialog: false,
  newCategoryName: "",
  selectedRecipe: {} as RecipeModel,
  createNewRecipe: true
});

onErrorCaptured((error) => {
  state.message = error.message;
  state.showToast = true;
})

const onNewRecipeSaved = (recipe:RecipeModel) => {
  state.accordianData
    .find((dataItem: { id: string, name: string, recipes: RecipeModel[] }) => dataItem.id == recipe.category_id)
    .recipes.push(recipe)
  onRecipeSelected(recipe)
}

const onRecipeSelected = async (recipe:RecipeModel) => {
  recipe.ingredients = await getRecipeIngredients(recipe.id)
  recipe.instructions = await getRecipeInstructions(recipe.id)
  state.selectedRecipe = recipe
  state.createNewRecipe = false
}

const getRecipeIngredients = async (recipe_id:string) => {
  try {
    const ingredientsResponse = await IngredientService.getRecipeIngredients(recipe_id);
    return ingredientsResponse.data
  } catch (error) {
      console.log("ERROR", error.response.data)
      throw(new Error(error.response.data))
  }
}

const getRecipeInstructions = async (recipe_id:string) => {
  try {
    const instructionsResponse = await InstructionService.getRecipeInstructions(recipe_id)
    return instructionsResponse.data;
  } catch (error) {
      console.log("ERROR", error.response.data)
      throw(new Error(error.response.data))
  }
}

const getCategories = async () => {
  try {
    const categoriesResponse = await CategoryService.getAll();
    if(categoriesResponse.status == 200) {
        state.categories = categoriesResponse.data
    }
  } catch (error) {
      console.log("ERROR", error.response.data)
      throw(new Error(error.response.data))
  }
}

const getRecipes = async () => {
  try {
    const response = await RecipeService.getAll();
    if(response.status == 200) {
        state.recipes = response.data
    }
  } catch (error) {
      console.log("ERROR", error.response.data)
      throw(new Error(error.response.data))
  }
}

const fetchData = async () => {
  await getCategories()
  await getRecipes()

  state.categories.forEach(category => {
    var recipes = state.recipes.filter((recipe) => { return recipe.category_id == category.id})
    state.accordianData.push(
      {
        ...category,
        recipes: recipes
      }
      
    )
  });

  state.loading = false;
}

const createCategory = async () => {
    try {
      const createCategoryResponse = await CategoryService.createCategory(state.newCategoryName)
      const newCategory = createCategoryResponse.data.category
      state.categories.push(newCategory)
      state.accordianData.push(
        {
          ...newCategory,
          recipes: []
        }
      )
    } catch (error) {
      console.log("ERROR", error.response.data.message)
      throw(new Error(error.response.data.message))
    }

    state.showNewCategoryDialog = false;
    state.newCategoryName = "";
}

fetchData();
</script>

<style scoped>
#app-container {
  /* background-color: red; */
  border: 1px solid black;
}

#header {
  text-align: center;
}

#categories-accordian {
    /* background-color: #e6ab0b */
}
</style>