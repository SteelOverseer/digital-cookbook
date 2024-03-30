<template>
  <v-container id="view-recipe-form">
    <v-row id="recipe-header">
      <div id="name-panel">
        <h2>{{ recipe.name }}</h2>
        <v-icon
          v-if="!recipe.is_favorite"
          icon="mdi-star-outline" 
          @click="$emit('toggleFavorite')"
        />
        <v-icon
          v-if="recipe.is_favorite"
          style="color: gold;"
          icon="mdi-star" 
          @click="$emit('toggleFavorite')"
        />
      </div>
      <v-icon icon="mdi-pencil" @click="$emit('editRecipe')" />
    </v-row>
    <v-row>
      <v-col>
        <h3>Ingredients</h3>
        <div v-for="ingredient in recipe.ingredients" :key="ingredient.id">
          - {{ ingredient.ingredient_text }}
        </div>
      </v-col>
      <v-col>
        <h3>Notes</h3>
        <div>{{ recipe.notes }}</div>
      </v-col>
    </v-row>
    <v-row>
      <v-col>
        <h3>Instructions</h3>
        <ol style="margin-left: 15px;">
          <li v-for="instruction in recipe.instructions" :key="instruction.id">
            {{ instruction.instruction_text }}
          </li>
        </ol>
      </v-col>
    </v-row>
  </v-container>
</template>
  
<script setup lang="ts">
import RecipeModel from '../models/Recipe/RecipeModel';

defineProps<{
  recipe:RecipeModel
}>()

</script>

<style scoped>
#view-recipe-form {
  background-color: #f6eee3; 

  #recipe-header {
    display: flex;
    flex-direction: row;
    justify-content: space-between;
    align-items: center;

    #name-panel {
      display: flex;
      flex-direction: row;
      align-items: center;
    }
  }
}
</style>