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
        <v-sheet class="pa-2 ma-2" id="categories-accordian">
          <Categories :categories="state.categories" />
        </v-sheet>
      </v-col>
      <v-col>
        <v-sheet class="pa-2 ma-2">
          <CreateRecipeForm :categories="state.categories" />
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
import CreateRecipeForm from './components/CreateRecipeForm.vue'
import { onErrorCaptured, reactive } from 'vue';
import CategoryService from './services/CategoryService';
import CategoryModel from './models/CategoryModel';

const state = reactive({
  categories: [] as CategoryModel[],
	showToast: false,
	message: ""
});

onErrorCaptured((error) => {
  console.log("LOOK MA I THREW")
  console.log(error)
  state.message = error.message;
  state.showToast = true;
})

const getCategories = async () => {
    const categoriesResponse = await CategoryService.getAll();

    if(categoriesResponse.data.status == "success") {
        state.categories = categoriesResponse.data.categories
    }
}

getCategories()
</script>

<style scoped>
#app-container {
  /* background-color: red; */
  border: 2px solid red;
}

#header {
  text-align: center;
}

#categories-accordian {
    background-color: #e6ab0b
}
</style>
./models/CategoryModel