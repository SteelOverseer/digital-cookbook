<script setup lang="ts">
// import Category from '../components/Category.vue'
import { defineProps } from 'vue'
import type { Category } from '../models/category'

const props = defineProps({
  categories: Array<Category>
})
</script>

<template>
    <div>
        <div class="accordion" id="category-accordion">
            <div class="accordion-item" v-for="category in props.categories" :key="category.categoryId">
                <h2 class="accordion-header" :id="`heading-category-${category.categoryId}`">
                    <button class="accordion-button" type="button" data-bs-toggle="collapse" :data-bs-target="`category-${category.categoryId}`" aria-expanded="true" :aria-controls="`category-${category.categoryId}`">
                        {{ category.name }}
                    </button>
                </h2>
                <div :id="`category-${category.categoryId}`" class="accordion-collapse collapse show" :aria-labelledby="`heading-category-${category.categoryId}`" data-bs-parent="#category-accordion">
                    <div class="accordion-body">
                        TEST
                    </div>
                </div>
            </div>
        </div>
        <button type="button" class="btn btn-outline-dark">New Category</button>
    </div>
</template>

<style scoped>
</style>
