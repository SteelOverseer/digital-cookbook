<template>
  <v-expansion-panels variant="accordion" id="categories-accordian">
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
      </v-expansion-panel-text>
    </v-expansion-panel>
  </v-expansion-panels>
</template>

<script setup lang="ts">
defineProps(['data'])
defineEmits(["selectRecipe"]);

</script>

<style scoped>
.expansion-title, .expansion-item {
  background-color: #f6eee3;
}
.expansion-item {
  cursor: pointer;
}
</style>