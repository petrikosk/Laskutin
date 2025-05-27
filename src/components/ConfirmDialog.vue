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
        <div v-if="icon" class="flex items-center justify-center w-8 h-8 mx-auto mb-4"
             :class="icon === 'danger' ? '' : 'rounded-full ' + iconBgClass">
          <div class="icon" :class="[iconClass, iconColorClass]"></div>
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

const iconClass = computed(() => {
  switch (props.icon) {
    case 'warning':
      return 'icon-warning'
    case 'danger':
      return 'icon-delete'
    default:
      return 'icon-info'
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