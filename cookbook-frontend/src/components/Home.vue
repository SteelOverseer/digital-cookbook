<template>
  <v-container fluid id="welcome">
    <h2>Welcome to the family cookbook</h2>
    <p>Pick a recipe from the list on the left, or start a new one.</p>
    <v-btn id="new-recipe-button" @click="$emit('newRecipe')">New Recipe</v-btn>
  </v-container>
</template>

<script setup lang="ts">
defineEmits(['newRecipe'])
</script>

<style scoped>
#welcome {
  background-color: #f6eee3;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 12px;
  text-align: center;
}
</style>
