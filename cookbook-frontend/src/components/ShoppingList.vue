<template>
  <v-container id="shopping-list">
    <h2>Shopping List</h2>
    <v-textarea
      v-model="list"
      rows="15"
    ></v-textarea>
    <div id="shopping-list-footer">
      <v-btn id="update-button" @click="$emit('updateShoppingList', list)">Update</v-btn>
      <v-btn id="clear-button" @click="$emit('clearShoppingList')">Clear</v-btn>
    </div>
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

  #shopping-list-footer {
    display: flex;
    flex-direction: row;
    justify-content: flex-end;

    #update-button {
      background-color: lightgreen;
      margin-right: 10px;
    }
  
    #clear-button {
      background-color: lightcoral;
    }
  }

}
</style>