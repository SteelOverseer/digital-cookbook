<template>
  <v-expansion-panels variant="accordion" id="categories-accordian">
    <v-expansion-panel id="favorites-panel">
      <v-expansion-panel-title class="expansion-title">
        <h3>
          <v-icon icon="mdi-star" size="small" class="favorite-star" />
          Favorites
        </h3>
      </v-expansion-panel-title>
      <v-expansion-panel-text
        v-for="recipe in favorites"
        :key="recipe.id"
        @click="$emit('selectRecipe', recipe)"
        class="expansion-item"
      >
        {{ recipe.name }}
      </v-expansion-panel-text>
      <v-expansion-panel-text
        v-if="favorites.length == 0"
        class="expansion-item empty-favorites"
      >
        Star a recipe to see it here.
      </v-expansion-panel-text>
    </v-expansion-panel>
    <v-expansion-panel 
      v-for="category in data"
      :key="category.id"
    >
      <v-expansion-panel-title class="expansion-title">
        <h3>
          {{ category.name }}
        </h3>
      </v-expansion-panel-title>
      <v-expansion-panel-text
        v-for="recipe in category.recipes"
        :key="recipe.id"
        @click="$emit('selectRecipe', recipe)"
        class="expansion-item"
      >
        {{ recipe.name }}
        <v-icon
          v-if="recipe.is_favorite"
          icon="mdi-star"
          size="x-small"
          class="favorite-star"
        />
      </v-expansion-panel-text>
    </v-expansion-panel>
  </v-expansion-panels>
</template>

<script setup lang="ts">
defineProps(['data', 'favorites'])
defineEmits(["selectRecipe"]);

</script>

<style scoped>
.expansion-title, .expansion-item {
  background-color: #f6eee3;
}
.expansion-item {
  cursor: pointer;
}
.favorite-star {
  color: gold;
}
.empty-favorites {
  cursor: default;
  font-style: italic;
}
</style>