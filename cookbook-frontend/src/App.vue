<template>
  <v-container id="app-container" fluid>
    <v-row no-gutters id="header">
      <v-col>
        <v-toolbar class="hidden-md-and-up">
          <v-app-bar-nav-icon />
          <v-toolbar-title @click="state.currentComponent = Home">Schultz Family Cookbook</v-toolbar-title>
          <v-spacer></v-spacer>
          <v-btn icon>
            <v-icon>mdi-magnify</v-icon>
          </v-btn>
        </v-toolbar>
        <h1
          id="desktop-header"
          class="hidden-sm-and-down" 
          @click="state.currentComponent = Home"
        >
          Schultz Family Cookbook
        </h1>
      </v-col>
    </v-row>
    <v-row no-gutters id="content-row">
      <v-col 
        cols="4"
        lg="3"
        xl="2"
        class="hidden-sm-and-down"
        id="action-column"
      >
        <div id="action-buttons">
          <v-row>
            <v-col cols="12" xl="6">
              <v-btn @click="state.showNewCategoryDialog = true">New Category</v-btn>
            </v-col>
            <v-col cols="12" xl="6">
              <v-btn @click="onNewRecipe()">New Recipe</v-btn>
            </v-col>
          </v-row>
        </div>
        <div 
          id="recipe-search"
          style="padding: 5px"
        >
          <v-text-field
            label="Search Recipes"
            prepend-icon="mdi-magnify"
            v-model="state.search"
            clearable
          >
          </v-text-field>
          <v-list
            style="background-color: #f6eee3; border-radius: 5px;"
            v-if="state.search != null && state.search != ''"
          >
            <v-list-item
              v-for="recipe in filteredRecipes"
              :key="recipe.id"
              :title="recipe.name"
              @click="onRecipeSelected(recipe)"
            ></v-list-item>
          </v-list>
        </div>
        <Categories 
          :data="state.accordianData" 
          :favorites="favoriteRecipes"
          @select-recipe="(recipe) => onRecipeSelected(recipe)" 
        />
      </v-col>
      <v-col id="main-column">
        <component 
          :is="state.currentComponent"
          v-bind="currentProps"
          @saved="(recipe: RecipeModel) => onNewRecipeSaved(recipe)"
          @editRecipe="onEditRecipe()"
          @cancelEdit="cancelEdit()"
          @toggleFavorite="toggleFavorite()"
          @selectRecipe="onRecipeSelected($event)"
          @addIngredientsToShoppingList="updateShoppingList()"
          @newRecipe="onNewRecipe()"
        />
      </v-col>
      <v-col
        cols="3"
        class="hidden-md-and-down"
        id="rail-column"
      >
        <ShoppingList
          :shoppingList="state.shoppingList"
          @updateShoppingList="state.shoppingList = $event"
          @clearShoppingList="state.shoppingList = ''; refresh();"
        />
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
    <v-card style="background-color: #f6eee3;">
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
import Home from './components/Home.vue'
import Categories from './components/Categories.vue'
import CreateRecipeForm from './components/CreateRecipeForm.vue'
import { computed, onErrorCaptured, reactive, type Component } from 'vue';
import CategoryService from './services/CategoryService';
import CategoryModel from './models/Category/CategoryModel';
import RecipeService from './services/RecipeService';
import RecipeModel from './models/Recipe/RecipeModel';
import Recipe from './components/Recipe.vue'
import IngredientService from './services/IngredientService';
import InstructionService from './services/InstructionService';
import CreateRecipeModel from './models/Recipe/CreateRecipeModel';
import ShoppingList from './components/ShoppingList.vue';

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
  selectedRecipe: new RecipeModel(),
  currentComponent: Home as Component,
  editRecipe: false,
  shoppingList: '',
  search: null as string | null
});

const filteredRecipes = computed(() => {
    const search = state.search
    if (search) {
      return state.recipes.filter((recipe) => {
        return search
          .toLowerCase()
          .split(" ")
          .every((v) => recipe.name.toLowerCase().includes(v))
      });
    } else {
        return state.recipes;
    }
});

const refresh = () => {
  window.location.reload();
}

const updateShoppingList = () => {
  state.selectedRecipe.ingredients.forEach((ingredient) => {
    state.shoppingList += (ingredient.ingredient_text + "\n")
  })
}

const favoriteRecipes = computed(() => {
  return state.recipes.filter((recipe) => {return recipe.is_favorite})
})

const currentProps = computed(() => {
  if((state.currentComponent as any).__name == 'CreateRecipeForm') {
    return { 
      categories: state.categories,
      recipe: state.selectedRecipe,
      isEdit: state.editRecipe
    }
  } else if ((state.currentComponent as any).__name == 'Recipe') {
    return { recipe: state.selectedRecipe }
  }
})

const toggleFavorite = async () => {
  try {
    const toggleFacoriteReq: CreateRecipeModel =  {
      is_favorite: !state.selectedRecipe.is_favorite
    }
    const resp =  await RecipeService.editRecipe(toggleFacoriteReq, state.selectedRecipe.id)
    const updateRecipe = resp.data;
    state.selectedRecipe.is_favorite = updateRecipe.is_favorite
    let recipe = state.recipes.find((recipe) => {return recipe.id == updateRecipe.id})
    if (recipe) {
      recipe.is_favorite = updateRecipe.is_favorite
    }
  } catch (error: any) {
      console.log("ERROR", error.response.data)
      throw(new Error(error.response.data))
  }
}

const cancelEdit = () => {
  state.editRecipe = false
  onNewRecipeSaved(state.selectedRecipe)
  state.currentComponent = Recipe
}

onErrorCaptured((error) => {
  state.message = error.message;
  state.showToast = true;
})

const onNewRecipe = () => {
  state.selectedRecipe = new RecipeModel()
  state.editRecipe = false
  state.currentComponent = CreateRecipeForm
}

const onEditRecipe = () => {
  state.editRecipe = true
  state.currentComponent = CreateRecipeForm
}

const onNewRecipeSaved = (recipe:RecipeModel) => {
  state.accordianData = [];
  fetchData()
  onRecipeSelected(recipe)
}

const onRecipeSelected = async (recipe:RecipeModel) => {
    Promise.all([await getRecipeIngredients(recipe.id), await getRecipeInstructions(recipe.id)])
      .then((data) => {
        recipe.ingredients = data[0]
        recipe.instructions = data[1]
        state.editRecipe = false
        state.selectedRecipe = recipe
        state.currentComponent = Recipe
        rememberRecipe(recipe.id)
      })
}

const LAST_RECIPE_KEY = 'cookbook.lastRecipeId'

const rememberRecipe = (id: string) => {
  try {
    localStorage.setItem(LAST_RECIPE_KEY, id)
  } catch {
    // storage unavailable; the welcome view shows next time
  }
}

const restoreLastRecipe = () => {
  try {
    const id = localStorage.getItem(LAST_RECIPE_KEY)
    if (!id) return
    const recipe = state.recipes.find((recipe) => recipe.id == id)
    if (recipe) {
      onRecipeSelected(recipe)
    } else {
      localStorage.removeItem(LAST_RECIPE_KEY)
    }
  } catch {
    // storage unavailable; stay on the welcome view
  }
}

const getRecipeIngredients = async (recipe_id:string) => {
  try {
    const ingredientsResponse = await IngredientService.getRecipeIngredients(recipe_id);
    return ingredientsResponse.data
  } catch (error: any) {
      console.log("ERROR", error.response.data)
      throw(new Error(error.response.data))
  }
}

const getRecipeInstructions = async (recipe_id:string) => {
  try {
    const instructionsResponse = await InstructionService.getRecipeInstructions(recipe_id)
    return instructionsResponse.data;
  } catch (error: any) {
      console.log("ERROR", error.response.data)
      throw(new Error(error.response.data))
  }
}

const getCategories = async () => {
  state.categories = []
  try {
    const categoriesResponse = await CategoryService.getAll();
    if(categoriesResponse.status == 200) {
        state.categories = categoriesResponse.data
    }
  } catch (error: any) {
      console.log("ERROR", error.response.data)
      throw(new Error(error.response.data))
  }
}

const getRecipes = async () => {
  state.recipes = []
  try {
    const response = await RecipeService.getAll();
    if(response.status == 200) {
        state.recipes = response.data
    }
  } catch (error: any) {
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
      const newCategory = createCategoryResponse.data
      state.categories.push(newCategory)
      state.accordianData.push(
        {
          ...newCategory,
          recipes: []
        }
      )
    } catch (error: any) {
      console.log("ERROR", error.response.data.message)
      throw(new Error(error.response.data.message))
    }

    state.showNewCategoryDialog = false;
    state.newCategoryName = "";
}

fetchData().then(restoreLastRecipe);
</script>

<style scoped>
#app-container {
  padding: var(--divider);
  min-height: 100vh;
  display: flex;
  flex-direction: column;

  #content-row {
    flex: 1;
  }

  #header {
    flex: none;
    text-align: center;
    
    #desktop-header {
      display: flex;
      align-items: center;
      justify-content: center;
      cursor: pointer; 
      background-color: #f6eee3;
      margin: 0 0 var(--divider);
      height: 125px;
      font-family: 'Satisfy', cursive;
      color: #333;
      font-size: clamp(32px, 6.5vw, 90px);
      line-height: 1;
      white-space: nowrap;
    }
  }

  #main-column {
    min-width: 0;
    display: flex;
    flex-direction: column;

    > * {
      flex: 1 1 auto;
    }
    background-color: #f6eee3;
  }

  #rail-column {
    margin-left: var(--divider);
    background-color: #f6eee3;
    display: flex;
    flex-direction: column;
  }

  #action-column {
    margin-right: var(--divider);
    background-color: #f6eee3;

    #action-buttons {
      display: flex;
      flex-direction: row;
      justify-content: space-between;
      border-bottom: var(--divider) solid var(--cookbook-blue);
  
      button {
        white-space: normal;
        width: -webkit-fill-available;
        background-color: #f6eee3;
      }
    }

    #recipe-search {
      background-color: #f6eee3;
      border-bottom: var(--divider) solid var(--cookbook-blue);
    }
  }

}

</style>