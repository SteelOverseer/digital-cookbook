<template>
  <v-container id="shopping-list">
    <h2>Shopping List</h2>
    <v-textarea
      :model-value="$props.shoppingList"
      rows="15"
    ></v-textarea>
  </v-container>
</template>

<script setup lang="ts">

defineProps<{
  shoppingList:string
}>()
</script>

<style scoped>
#shopping-list {
  background-color: #f6eee3;
  border-radius: 15px;
}
</style>