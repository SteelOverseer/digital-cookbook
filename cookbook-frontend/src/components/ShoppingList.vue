<template>
  <v-container id="shopping-list">
    <h2>Shopping List</h2>
    <v-textarea
      v-model="list"
      rows="15"
    ></v-textarea>
  </v-container>
</template>

<script setup lang="ts">
import { ref } from 'vue';

const props = defineProps<{
  shoppingList:string
}>()

const list = ref(props.shoppingList)
</script>

<style scoped>
#shopping-list {
  background-color: #f6eee3;
  border-radius: 15px;
}
</style>