<template>
    <v-expansion-panels variant="popout">
        <v-expansion-panel 
            title="Title"
            v-for="i in 3"
            :key="i"
            text="Lorem ipsum dolor sit amet consectetur adipisicing elit. Commodi, ratione debitis quis est labore voluptatibus! Eaque cupiditate minima"
        >
        </v-expansion-panel>
    </v-expansion-panels>
</template>

<script setup lang="ts">


</script>

<style scoped>

</style>
