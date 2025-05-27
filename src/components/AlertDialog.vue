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
        <div v-if="icon" class="flex items-center justify-center w-8 h-8 mx-auto mb-4 rounded-full"
             :class="iconBgClass">
          <div class="icon" :class="[iconClass, iconColorClass]"></div>
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

const iconClass = computed(() => {
  switch (props.icon) {
    case 'success':
      return 'icon-success'
    case 'warning':
      return 'icon-warning'
    case 'error':
      return 'icon-error'
    default:
      return 'icon-info'
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