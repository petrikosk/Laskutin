<template>
  <div
    v-if="show"
    class="modal-overlay"
    @click="close"
  >
    <div
      class="modal-content"
      @click.stop
    >
      <div class="mt-3">
        <div v-if="icon" class="flex items-center justify-center w-12 h-12 mx-auto mb-4 rounded-full"
             :class="iconBgClass">
          <svg v-if="icon === 'success'" class="w-6 h-6" :class="iconColorClass" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
          </svg>
          <svg v-else-if="icon === 'error'" class="w-6 h-6" :class="iconColorClass" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
          </svg>
          <svg v-else-if="icon === 'warning'" class="w-6 h-6" :class="iconColorClass" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.732-.833-2.5 0L4.268 18.5c-.77.833.192 2.5 1.732 2.5z" />
          </svg>
          <svg v-else class="w-6 h-6" :class="iconColorClass" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
          </svg>
        </div>
        
        <h3 class="text-lg font-medium text-gray-900 mb-4 text-center">
          {{ title }}
        </h3>
        
        <div class="text-sm text-gray-600 mb-6 whitespace-pre-line text-center">
          {{ message }}
        </div>
        
        <div class="flex justify-center">
          <button
            @click="close"
            class="btn btn-primary"
          >
            {{ closeText }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

interface Props {
  show: boolean
  title: string
  message: string
  closeText?: string
  type?: 'info' | 'success' | 'warning' | 'error'
  icon?: 'info' | 'success' | 'warning' | 'error'
}

const props = withDefaults(defineProps<Props>(), {
  closeText: 'OK',
  type: 'info',
  icon: 'info'
})

const emit = defineEmits<{
  close: []
}>()

const iconBgClass = computed(() => {
  switch (props.icon) {
    case 'success':
      return 'bg-green-100'
    case 'warning':
      return 'bg-yellow-100'
    case 'error':
      return 'bg-red-100'
    default:
      return 'bg-blue-100'
  }
})

const iconColorClass = computed(() => {
  switch (props.icon) {
    case 'success':
      return 'text-green-600'
    case 'warning':
      return 'text-yellow-600'
    case 'error':
      return 'text-red-600'
    default:
      return 'text-blue-600'
  }
})

const close = () => {
  emit('close')
}
</script>