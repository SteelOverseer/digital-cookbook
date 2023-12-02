<template>
  <v-container>
    <v-row no-gutters>
      <v-col>Create Recipe</v-col>
    </v-row>
    <v-row>
      <v-col>
        <form>
          <v-text-field
            v-model="state.name"
            label="Name"
          ></v-text-field>
          <v-select
            label="Category"
            :items="props.categories"
            item-title="name"
            item-value="id"
            v-model="state.category"
          ></v-select>
          <v-textarea
            v-model="state.ingredients"
            label="Ingredients"
          ></v-textarea>
          <!-- <v-list two-line>
            <draggable v-model="state.items">
              <template v-for="(v, i) in state.items" :key="v.ID">
                <v-list-item>
                  <v-list-item-title>
                    {{ v.Title }}
                  </v-list-item-title>

                </v-list-item> -->
                <!-- <v-list-tile  avatar>
                  <v-list-tile-avatar color="grey">
                    <span style="user-select:none;">{{ i+1 }}</span>
                  </v-list-tile-avatar>
                  <v-list-tile-content>
                    <v-list-tile-title v-html="v.Title"></v-list-tile-title>
                    <v-list-tile-sub-title v-html="v.Subtitle"></v-list-tile-sub-title>
                  </v-list-tile-content> -->
<!--                   
                  <v-list-tile-action v-if="editing">
                    <v-btn @click="remove(i)" icon><v-icon>close</v-icon></v-btn>
                  </v-list-tile-action>
-->
                <!-- </v-list-tile> -->
              <!-- </template>
            </draggable> -->
          <!-- </v-list> -->
          <v-textarea
            v-model="state.notes"
            label="Notes"
          ></v-textarea>
          <v-btn
            class="me-4"
            @click="submit"
          >
            submit
          </v-btn>
          <v-btn>
            clear
          </v-btn>
        </form>
      </v-col>
    </v-row>
  </v-container>
  
</template>

<script setup lang="ts">
import { reactive } from 'vue';
import CreateRecipeModel from '../models/CreateRecipeModel';
import RecipeService from '../services/RecipeService';

const props = defineProps(['categories'])

const state = reactive({
  name: "",
  notes: "",
  ingredients: "",
  category: "",
  items: [
        {"ID":1,"Title":"Fact sheets, brochures, educational materials","Ordering":1,"Subtitle":""},
        {"ID":2,"Title":"HHS Clearance submission","Ordering":2,"Subtitle":"(for campaigns, campaign products, and/or videos)"},
        {"ID":3,"Title":"Abstracts","Ordering":3,"Subtitle":""},
        {"ID":4,"Title":"Non-media Blog/blog posts","Ordering":4,"Subtitle":"(internal or external)"},
        {"ID":5,"Title":"CDC Connects articles","Ordering":5,"Subtitle":""},
        {"ID":6,"Title":"CDC.gov features","Ordering":6,"Subtitle":""},
        {"ID":7,"Title":"Logo use/branding","Ordering":7,"Subtitle":""},
        {"ID":8,"Title":"External newsletters","Ordering":8,"Subtitle":""},
        {"ID":9,"Title":"Infographics","Ordering":9,"Subtitle":""},
        {"ID":10,"Title":"Other","Ordering":10,"Subtitle":""},
        {"ID":11,"Title":"Partnership plans/strategies","Ordering":11,"Subtitle":"(health communications focused)"},
        {"ID":12,"Title":"Podcast/vodcasts","Ordering":12,"Subtitle":""},
        {"ID":13,"Title":"PowerPoint Presentations","Ordering":13,"Subtitle":""},
        {"ID":14,"Title":"Scripts","Ordering":14,"Subtitle":"(for video, podcasts, or webcasts)"},
        {"ID":15,"Title":"Social media plans/proactive messages","Ordering":15,"Subtitle":""},
        {"ID":16,"Title":"Twitter events, plans, & tweets","Ordering":16,"Subtitle":""},
        {"ID":17,"Title":"Video & training materials","Ordering":17,"Subtitle":"(not requiring HHS clearance)"},
        {"ID":18,"Title":"Web banners, buttons, and badges","Ordering":18,"Subtitle":""},
        {"ID":19,"Title":"Web content on CDC.gov","Ordering":19,"Subtitle":"(top-level topic pages)"},
        {"ID":20,"Title":"Press materials ","Ordering":20,"Subtitle":"(i.e. media advisories, press releases, press conference speeches, media fact sheets, letters to the editor, op-eds, rollout plans)"},
        {"ID":21,"Title":"Communication strategies","Ordering":21,"Subtitle":"(for highly visible or controversial issues or as requested by OD)"},
        {"ID":22,"Title":"Key points","Ordering":22,"Subtitle":""},
        {"ID":23,"Title":"Cross-center global health communication content ","Ordering":23,"Subtitle":"(i.e. newsletter articles, blogs, success stories)"}]
    
});

const submit = async () => {
  console.log("IN SUBMIT")
  console.log(state.category)
  console.log('state:', state)

  const request:CreateRecipeModel =  {
    name: state.name,
    category_id: state.category,
    notes: state.notes,
    ingredients: state.ingredients
  }

  try {
    let resp = await RecipeService.createRecipe(request);
    console.log('resp:', resp)
  } catch (error) {
      console.log("ERROR", error.response.data.message)
      throw(new Error(error.response.data.message))
  }
  
}
</script>

<style scoped>

</style>
