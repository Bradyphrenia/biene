<template>
  <button class="button is-primary">
    <span>
      <slot> {{ label }}</slot>
    </span>
  </button>
</template>

<script setup>
const props = defineProps({
  label: {
    type: String,
    required: true,
  },
})
</script>
