<template>
  <v-container fluid id="shopping-list">
    <h2>Shopping List</h2>
    <v-textarea
      v-model="list"
      rows="6"
      no-resize
      :placeholder="placeholder"
      persistent-placeholder
    ></v-textarea>
    <div id="shopping-list-footer">
      <v-tooltip text="Download list">
        <template v-slot:activator="{ props }">
          <v-btn
            v-bind="props"
            id="download-button"
            aria-label="Download list"
            :disabled="!list.trim()"
            @click="downloadList"
          >
            <v-icon icon="mdi-download" />
          </v-btn>
        </template>
      </v-tooltip>
      <v-btn id="clear-button" @click="list = ''">Clear</v-btn>
    </div>
  </v-container>
</template>

<script setup lang="ts">
const list = defineModel<string>({ default: '' })

const localDate = () => {
  const now = new Date()
  const pad = (n: number) => String(n).padStart(2, '0')
  return `${now.getFullYear()}-${pad(now.getMonth() + 1)}-${pad(now.getDate())}`
}

const downloadList = () => {
  const blob = new Blob([list.value], { type: 'text/plain;charset=utf-8' })
  const url = URL.createObjectURL(blob)
  const link = document.createElement('a')
  link.href = url
  link.download = `ShoppingList-${localDate()}.txt`
  link.click()
  URL.revokeObjectURL(url)
}

const placeholder =
  'Type items, one per line, like:\n2 cups flour\n1 dozen eggs\n\nOr use the cart icon on a recipe to add all its ingredients.'
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
    gap: 10px;

    #download-button {
      background-color: lightgreen;
    }

    #clear-button {
      background-color: lightcoral;
    }
  }

}
</style>