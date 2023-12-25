<template>
  <Icon :icon="icon" height="auto"/>
</template>

<script setup >
import { Icon } from '@iconify/vue';

const props = defineProps({
  icon: {
    type: Object || String,
    required: true,
  },
})
</script>
