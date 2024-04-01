<template>
  <v-container id="app-container">
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
    <v-row no-gutters>
      <v-col 
        cols="3" 
        sm="2"
        class="hidden-sm-and-down"
        id="action-column"
      >
        <div id="action-buttons">
          <v-row>
            <v-col>
              <v-btn @click="state.showNewCategoryDialog = true">New Category</v-btn>
            </v-col>
            <v-col>
              <v-btn @click="state.selectedRecipe = new RecipeModel(); state.currentComponent = CreateRecipeForm;">New Recipe</v-btn>
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
          @select-recipe="(recipe) => onRecipeSelected(recipe)" 
        />
      </v-col>
      <v-col>
          <component 
            :is="state.currentComponent"
            v-bind="currentProps"
            @saved="(recipe) => onNewRecipeSaved(recipe)"
            @editRecipe="onEditRecipe()"
            @cancelEdit="cancelEdit()"
            @toggleFavorite="toggleFavorite()"
            @selectRecipe="onRecipeSelected($event)"
            @addIngredientsToShoppingList="updateShoppingList()"
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
import { computed, onErrorCaptured, reactive } from 'vue';
import CategoryService from './services/CategoryService';
import CategoryModel from './models/Category/CategoryModel';
import RecipeService from './services/RecipeService';
import RecipeModel from './models/Recipe/RecipeModel';
import Recipe from './components/Recipe.vue'
import IngredientService from './services/IngredientService';
import InstructionService from './services/InstructionService';
import CreateRecipeModel from './models/Recipe/CreateRecipeModel';

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
  currentComponent: Home,
  editRecipe: false,
  shoppingList: '',
  search: null
});

const filteredRecipes = computed(() => {
    if (state.search) {
      return state.recipes.filter((recipe) => {
        return state.search
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
  if(state.currentComponent.__name == 'CreateRecipeForm') {
    return { 
      categories: state.categories,
      recipe: state.selectedRecipe,
      isEdit: state.editRecipe
    }
  } else if (state.currentComponent.__name == 'Recipe') {
    return { recipe: state.selectedRecipe }
  } else if (state.currentComponent.__name == 'Home') {
    return {
      recipes: favoriteRecipes.value,
      shoppingList: state.shoppingList
    }
  } else if (state.currentComponent.__name == 'Favorites') {
    return {
      recipes: favoriteRecipes.value,
    }
  } else if (state.currentComponent.__name == 'ShoppingList') {
    return {
      shoppingList: state.shoppingList
    }
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
    recipe.is_favorite = updateRecipe.is_favorite
  } catch (error) {
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
      })
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
  state.categories = []
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
  state.recipes = []
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
      const newCategory = createCategoryResponse.data
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
  #header {
    text-align: center;
    
    #desktop-header {
      display: flex;
      align-items: center;
      justify-content: center;
      cursor: pointer; 
      background-color: #f6eee3;
      margin-bottom: 10px;
      height: 125px;
      font-family: 'dobkinplain';
      color: #333;
      font-size: 90px;
    }
  }

  #action-column {
    margin-right: 10px;

    #action-buttons {
      display: flex;
      flex-direction: row;
      justify-content: space-between;
  
      button {
        width: -webkit-fill-available;
        background-color: #f6eee3;
      }
    }

    #recipe-search {
      background-color: #f6eee3;
      margin-top: 10px;
      margin-bottom: 10px;
      border-radius: 5px;
    }
  }

}

</style>