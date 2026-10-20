<template>
  <v-container fluid id="shopping-list">
    <h2>Shopping List</h2>
    <v-textarea
      v-model="list"
      rows="6"
      no-resize
    ></v-textarea>
    <div id="shopping-list-footer">
      <v-btn id="update-button" @click="$emit('updateShoppingList', list)">Update</v-btn>
      <v-btn id="clear-button" @click="$emit('clearShoppingList')">Clear</v-btn>
    </div>
  </v-container>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue';

const props = defineProps<{
  shoppingList:string
}>()

const list = ref(props.shoppingList)

watch(() => props.shoppingList, (value) => {
  list.value = value
})
</script>

<style scoped>
#shopping-list {
  background-color: #f6eee3;
  display: flex;
  flex-direction: column;
  height: 100%;

  :deep(.v-textarea) {
    flex: 1;
  }

  :deep(.v-textarea .v-input__control),
  :deep(.v-textarea .v-field),
  :deep(.v-textarea .v-field__field),
  :deep(.v-textarea textarea) {
    height: 100%;
  }

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