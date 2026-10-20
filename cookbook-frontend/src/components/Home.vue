<template>
  <v-row no-gutters>
    <v-col cols="12" lg="6" class="home-left">
      <FavoriteRecipes 
        :recipes="recipes"
        @selectRecipe="$emit('selectRecipe', $event)"
      />
    </v-col>
    <!-- <v-col cols="12" lg="6" class="hidden-sm-and-down home-right">
      <ShoppingList
        :shoppingList="shoppingList"
        @updateShoppingList="$emit('updateShoppingList', $event)"
        @clearShoppingList="$emit('clearShoppingList')"
      />
    </v-col> -->
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
.home-right {
  display: flex;
  flex-direction: column;
}

.home-left {
  border-bottom: var(--divider) solid var(--cookbook-blue);
}

@media (min-width: 1145px) {
  .home-right {
  display: flex;
  flex-direction: column;
}

.home-left {
    border-bottom: 0;
    border-right: var(--divider) solid var(--cookbook-blue);
  }
}
</style>