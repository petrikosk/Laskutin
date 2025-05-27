<template>
  <div
    v-if="show"
    class="modal-overlay"
    @click="cancel"
  >
    <div
      class="modal-content"
      @click.stop
    >
      <div class="mt-3">
        <div v-if="icon" class="flex items-center justify-center w-12 h-12 mx-auto mb-4 rounded-full"
             :class="iconBgClass">
          <svg v-if="icon === 'warning'" class="w-6 h-6" :class="iconColorClass" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.732-.833-2.5 0L4.268 18.5c-.77.833.192 2.5 1.732 2.5z" />
          </svg>
          <svg v-else-if="icon === 'danger'" class="w-6 h-6" :class="iconColorClass" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
          </svg>
          <svg v-else class="w-6 h-6" :class="iconColorClass" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8.228 9c.549-1.165 2.03-2 3.772-2 2.21 0 4 1.343 4 3 0 1.4-1.278 2.575-3.006 2.907-.542.104-.994.54-.994 1.093m0 3h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
          </svg>
        </div>
        
        <h3 class="text-lg font-medium text-gray-900 mb-4 text-center">
          {{ title }}
        </h3>
        
        <div class="text-sm text-gray-600 mb-6 whitespace-pre-line text-center">
          {{ message }}
        </div>
        
        <div class="flex justify-center space-x-3">
          <button
            @click="cancel"
            class="btn btn-secondary"
          >
            {{ cancelText }}
          </button>
          <button
            @click="confirm"
            class="btn"
            :class="confirmButtonClass"
          >
            {{ confirmText }}
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
  confirmText?: string
  cancelText?: string
  type?: 'info' | 'warning' | 'danger'
  icon?: 'info' | 'warning' | 'danger'
}

const props = withDefaults(defineProps<Props>(), {
  confirmText: 'Vahvista',
  cancelText: 'Peruuta',
  type: 'info',
  icon: 'info'
})

const emit = defineEmits<{
  confirm: []
  cancel: []
}>()

const iconBgClass = computed(() => {
  switch (props.icon) {
    case 'warning':
      return 'bg-yellow-100'
    case 'danger':
      return 'bg-red-100'
    default:
      return 'bg-blue-100'
  }
})

const iconColorClass = computed(() => {
  switch (props.icon) {
    case 'warning':
      return 'text-yellow-600'
    case 'danger':
      return 'text-red-600'
    default:
      return 'text-blue-600'
  }
})

const confirmButtonClass = computed(() => {
  switch (props.type) {
    case 'warning':
      return 'btn-warning'
    case 'danger':
      return 'btn-danger'
    default:
      return 'btn-primary'
  }
})

const confirm = () => {
  emit('confirm')
}

const cancel = () => {
  emit('cancel')
}
</script>