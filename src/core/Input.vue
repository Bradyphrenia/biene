<template>
  <div class="form-group" :class="{ 'is-inline': inline }">
    <slot name="label">
      <label v-if="label">{{ label }}</label>
    </slot>
    <div class="input-container">
      <input
        v-if="inputType !== 'textarea'"
        :type="inputType"
        :placeholder="placeholder ? placeholder : undefined"
        :disabled="disabled"
        class="form-control"
        ref="input"
        v-model="inputValue"
        @blur=handleBlur />
      <textarea
        v-else
        v-model="inputValue">
      </textarea>
        <b></b>
    </div>
  </div>
</template>

<script setup>
import { ref, watch } from "vue";

const props = defineProps({
  modelValue: {
    type: String,
    required: true,
  },
  label: {
    type: String,
    default: () => null,
  },
  type: {
    type: String,
    default: () => 'text',
  },
  inline: {
    type: Boolean,
    default: false
  },
  placeholder: {
    type: String,
    default: undefined
  },
  disabled: {
    type: Boolean,
    default: false
  }
});

const emit = defineEmits(['update:modelValue', 'blur'])

const inputValue = ref(props.modelValue)
watch(inputValue, (newVal) => {
  emit('update:modelValue', newVal)
})

const inputType = ref(props.type)

function handleBlur (...args) {
  for (const handler of [handleFocus]) {
    try {
      handler(...args)
    } catch (error) {
      console.log(error)
    }
  }
}

const isFocused = ref(false)

function handleFocus (event) {
  emit(event.type, event)
  isFocused.value = event.type === 'focus'
}
</script>
