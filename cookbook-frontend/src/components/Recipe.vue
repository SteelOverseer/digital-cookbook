<template>
  <h2>{{ recipe.name }}</h2>
  <v-container>
    <v-row>
      <v-col>
        <h3>Ingredients</h3>
        <div v-for="ingredient in recipe.ingredients" :key="ingredient.id">
          {{ ingredient.ingredient_text }}
        </div>
        <!-- <ul>
          <li v-for="(ingredient, index) in recipe.ingredients" :key="index">{{ ingredient }}</li>
        </ul> -->
      </v-col>
      <v-col>
        <h3>Notes</h3>
        <div>{{ recipe.notes }}</div>
      </v-col>
    </v-row>
    <v-row>
      <v-col>
        <h3>Steps</h3>
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

</style>