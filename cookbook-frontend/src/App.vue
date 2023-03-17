<script setup lang="ts">
import { RouterLink, RouterView } from 'vue-router'
import CategoriesView from './views/CategoriesView.vue'
import { reactive, onMounted } from 'vue'
import axios from 'axios'

const state = reactive({ categories: [] })

onMounted(async () => {
    try {
      const res = await axios.get(`http://localhost:3000/categories`);
      state.categories = res.data;
      console.log(state.categories)
    } catch (error) {
      console.log(error);
    }
})
</script>

<template>
<div class="container">
    <div class="row">
        <div class="col"></div>
        <div class="col-6 text-center">
            <h1>
                Schultz Family Cookbook
            </h1>
        </div>
        <div class="col"></div>
    </div>
    <div class="row">
        <div class="col">
            <CategoriesView :categories="state.categories" />
        </div>
        <div class="col-8 text-center">
            recipe view
        </div>
        <div class="col">ingredients list</div>
    </div>
</div>
</template>

<style scoped>

</style>
