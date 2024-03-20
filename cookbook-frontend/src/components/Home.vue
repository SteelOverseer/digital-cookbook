<template>
  <v-row>
    <v-col>
      <FavoriteRecipes 
        :recipes="recipes"
        @selectRecipe="$emit('selectRecipe', $event)"
      />
    </v-col>
    <v-col style="border: 1px solid black;">
      <h2>Shopping List</h2>
    </v-col>
  </v-row>
</template>

<script setup lang="ts">
import RecipeModel from '../models/Recipe/RecipeModel';
import FavoriteRecipes from './FavoriteRecipes.vue';

defineProps<{
  recipes:RecipeModel[]
}>()
</script>

<style scoped>

</style>