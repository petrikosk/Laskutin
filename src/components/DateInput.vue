<template>
  <VueDatePicker
    v-model="dateValue"
    :locale="locale"
    :format="format"
    :enable-time-picker="enableTimePicker"
    :placeholder="placeholder"
    :required="required"
    :disabled="disabled"
    :class="inputClass"
    @update:model-value="handleDateChange"
  />
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'
import VueDatePicker from '@vuepic/vue-datepicker'
import '@vuepic/vue-datepicker/dist/main.css'

interface Props {
  modelValue?: string | Date | null
  locale?: string
  format?: string
  enableTimePicker?: boolean
  placeholder?: string
  required?: boolean
  disabled?: boolean
  inputClass?: string
}

interface Emits {
  (e: 'update:modelValue', value: string | Date | null): void
}

const props = withDefaults(defineProps<Props>(), {
  locale: 'fi',
  format: 'dd.MM.yyyy',
  enableTimePicker: false,
  placeholder: 'Valitse päivämäärä',
  required: false,
  disabled: false,
  inputClass: ''
})

const emit = defineEmits<Emits>()

const dateValue = ref<Date | null>(null)

// Convert string to Date when props.modelValue changes
watch(() => props.modelValue, (newValue) => {
  if (newValue) {
    if (typeof newValue === 'string') {
      dateValue.value = new Date(newValue)
    } else {
      dateValue.value = newValue
    }
  } else {
    dateValue.value = null
  }
}, { immediate: true })

const handleDateChange = (newDate: Date | null) => {
  dateValue.value = newDate
  // Convert Date to ISO string for backend compatibility
  if (newDate) {
    const isoString = newDate.toISOString().split('T')[0]
    emit('update:modelValue', isoString)
  } else {
    emit('update:modelValue', null)
  }
}

</script>