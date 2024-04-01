<template>
  <v-row>
    <v-col>
      <FavoriteRecipes 
        :recipes="recipes"
        @selectRecipe="$emit('selectRecipe', $event)"
      />
    </v-col>
    <v-col class="hidden-sm-and-down">
      <ShoppingList
        :shoppingList="shoppingList"
        @updateShoppingList="$emit('updateShoppingList', $event)"
        @clearShoppingList="$emit('clearShoppingList')"
      />
    </v-col>
  </v-row>
</template>

<script setup lang="ts">
import RecipeModel from '../models/Recipe/RecipeModel';
import FavoriteRecipes from './FavoriteRecipes.vue';
import ShoppingList from './ShoppingList.vue';

defineProps<{
  recipes:RecipeModel[],
  shoppingList:string
}>()
</script>

<style scoped>

</style>