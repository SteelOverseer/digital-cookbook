<template>
  <h2>{{ recipe.name }}</h2>
  <v-container>
    <v-row>
      <v-col>
        <h3>Ingredients</h3>
        <div></div>
        <!-- <ul>
          <li v-for="(ingredient, index) in recipe.ingredients" :key="index">{{ ingredient }}</li>
        </ul> -->
      </v-col>
      <v-col>
        <h3>Notes</h3>
        <div>{{ recipe.notes }}</div>
      </v-col>
    </v-row>
    <v-row>
      <v-col>
        <h3>Steps</h3>
      </v-col>
    </v-row>
  </v-container>
</template>
  
<script setup lang="ts">
defineProps(['recipe'])

</script>

<style scoped>

</style>