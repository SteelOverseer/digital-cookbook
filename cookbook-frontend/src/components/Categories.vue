<template>
  <v-expansion-panels variant="accordion">
    <v-expansion-panel 
      v-for="category in data"
      :key="category.id"
    >
      <v-expansion-panel-title>{{ category.name }}</v-expansion-panel-title>
      <v-expansion-panel-text
        v-for="recipe in category.recipes"
        :key="recipe.id"
        @click="$emit('selectRecipe', recipe)"
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
/* #categories-accordian {
    background-color: #e6ab0b
} */
</style>