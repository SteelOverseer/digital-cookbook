<template>
  <v-container id="app-container">
    <v-row no-gutters id="header">
      <v-col>
		<!-- <v-toolbar style="background-color: grey; color:red">
			<v-app-bar-nav-icon></v-app-bar-nav-icon>
			<v-toolbar-title>Schultz Family Cookbook</v-toolbar-title>
		</v-toolbar> -->
        <h1>Schultz Family Cookbook</h1>
      </v-col>
    </v-row>
    <v-row no-gutters>
      <v-col cols="2">
        <v-sheet class="pa-2 ma-2">
          <Categories />
        </v-sheet>
      </v-col>
      <v-col>
        <v-sheet class="pa-2 ma-2">
          
        </v-sheet>
      </v-col>
    </v-row>
    <v-snackbar
      v-model="state.showToast"
      multi-line
	  location="top right"
    >
      {{ state.message }}
      <template v-slot:actions>
        <v-btn
          color="red"
          variant="text"
          @click="state.showToast = false"
        >
          Close
        </v-btn>
      </template>
    </v-snackbar>
  </v-container>
</template>

<script setup lang="ts">
import Categories from './components/Categories.vue'
import { onErrorCaptured, reactive } from 'vue';

const state = reactive({
	showToast: false,
	message: ""
});

onErrorCaptured((error) => {
  console.log("LOOK MA I THREW")
  console.log(error)
  state.message = error.message;
  state.showToast = true;
})
</script>

<style scoped>
#app-container {
  background-color: blue;
}

#header {
  text-align: center;
}
</style>
