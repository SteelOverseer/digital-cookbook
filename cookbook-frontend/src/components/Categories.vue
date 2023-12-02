<template>
    <v-btn variant="tonal" @click="state.showDialog = true">New Category</v-btn>
    <v-expansion-panels variant="accordion">
        <v-expansion-panel 
            v-for="category in categories"
            :key="category.id"
        >
            <v-expansion-panel-title>{{ category.name }}</v-expansion-panel-title>
            <v-expansion-panel-text
                v-for="i in 10"
                :key="i"
            >
                Recipe {{ i }}
            </v-expansion-panel-text>
        </v-expansion-panel>
    </v-expansion-panels>

    <v-dialog
      v-model="state.showDialog"
      persistent
      width="300"
    >
      <v-card>
        <v-card-title>
          <span class="text-h5">Create Category</span>
        </v-card-title>
        <v-card-text>
          <v-container>
            <v-row>
              <v-col
                cols="12"
                sm="8"
                md="10"
              >
                <v-text-field
                  label="Category Name*"
                  required
                  v-model="state.newCategoryName"
                ></v-text-field>
              </v-col>
            </v-row>
          </v-container>
          <small>*indicates required field</small>
        </v-card-text>
        <v-card-actions>
          <v-spacer></v-spacer>
          <v-btn
            color="blue-darken-1"
            variant="text"
            @click="state.showDialog = false"
          >
            Close
          </v-btn>
          <v-btn
            color="blue-darken-1"
            variant="text"
            @click="createCategory()"
          >
            Save
          </v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>
</template>

<script setup lang="ts">
import { reactive } from 'vue';
import CategoryService from '../services/CategoryService';
// import CategoryModel from '../models/CategoryModel';

const props = defineProps(['categories'])

const state = reactive({
//   categories: [] as Category[],
  showDialog: false,
  newCategoryName: "",
  loading: false
});

// const getCategories = async () => {
//     const categoriesResponse = await CategoryService.getAll();

//     if(categoriesResponse.data.status == "success") {
//         state.categories = categoriesResponse.data.categories
//     }
// }

const createCategory = async () => {
    console.log("i am here", state.newCategoryName)
    try {
        const createCategoryResponse = await CategoryService.createCategory(state.newCategoryName)
        props.categories.push(createCategoryResponse.data.category)
    } catch (error) {
        console.log("ERROR", error.response.data.message)
        throw(new Error(error.response.data.message))
    }

    // console.log('createCategoryResponse:', createCategoryResponse)
    // if(createCategoryResponse.data.status == "success") {
    // }
    state.showDialog = false;
    state.newCategoryName = "";
}

// getCategories()

</script>

<style scoped>
/* #categories-accordian {
    background-color: #e6ab0b
} */
</style>
../models/CategoryModel