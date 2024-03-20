<template>
  <v-container id="favorite-recipes">
    <h2>Favorites</h2>
    <v-list 
      style="background-color: #f6eee3;"
    >
      <v-list-item
        v-for="recipe in recipes"
        :key="recipe.id"
        :title="recipe.name"
        @click="$emit('selectRecipe', recipe)"
      >
      </v-list-item>
    </v-list>
  </v-container>
</template>

<script setup lang="ts">
import RecipeModel from '../models/Recipe/RecipeModel';

defineProps<{
  recipes:RecipeModel[]
}>()
</script>

<style scoped>
#favorite-recipes {
    background-color: #f6eee3;
    border-radius: 15px;
}
</style>