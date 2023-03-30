<template>
  <div class="columns">
    <div class="column">
      {{ label }}
    </div>
    <div class="column control">
      <slot />
    </div>
  </div>
</template>

<script setup>

const props = defineProps({
  label: {
    type: String,
    required: true,
  },
})

</script>
