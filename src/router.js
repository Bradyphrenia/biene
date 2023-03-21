import Greet from "./components/Greet.vue";
import ReviewVue from "./components/ReviewView.vue";

import { createRouter, createWebHashHistory } from 'vue-router'

const routes = [
  {
    path: '/test',
    component: Greet,
  },
  {
    path: '/test2',
    component: ReviewVue,
  }
]

const router = createRouter({
  history: createWebHashHistory(),
  routes,
})

export default router
