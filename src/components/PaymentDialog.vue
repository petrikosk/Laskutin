<template>
  <div v-if="show" class="modal-overlay" @click="cancel">
    <div class="modal-content" @click.stop>
      <div class="mt-3">
        <div class="mx-auto flex items-center justify-center h-12 w-12 rounded-full bg-green-100">
          <svg class="h-6 w-6 text-green-600" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
          </svg>
        </div>
        <div class="mt-3 text-center sm:mt-5">
          <h3 class="text-lg leading-6 font-medium text-gray-900">
            Merkitse lasku maksetuksi
          </h3>
          <div class="mt-2">
            <p class="text-sm text-gray-500">
              Anna maksupäivämäärä laskulle {{ invoiceReference }}
            </p>
          </div>
          <div class="mt-4">
            <label class="form-label">Maksupäivä *</label>
            <DateInput
              v-model="paymentDate"
              :required="true"
              placeholder="Valitse maksupäivä"
              input-class="form-input"
            />
          </div>
        </div>
      </div>
      <div class="mt-5 sm:mt-6 sm:grid sm:grid-cols-2 sm:gap-3 sm:grid-flow-row-dense">
        <button
          @click="confirm"
          :disabled="!paymentDate"
          type="button"
          class="btn btn-success"
          :class="{ 'opacity-50 cursor-not-allowed': !paymentDate }"
        >
          Merkitse maksetuksi
        </button>
        <button
          @click="cancel"
          type="button"
          class="btn btn-outline"
        >
          Peruuta
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'
import DateInput from './DateInput.vue'

interface Props {
  show: boolean
  invoiceReference?: string
}

interface Emits {
  (e: 'confirm', paymentDate: string): void
  (e: 'cancel'): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

const paymentDate = ref<string>('')

// Set default payment date to today when dialog opens
watch(() => props.show, (isVisible) => {
  if (isVisible) {
    paymentDate.value = new Date().toISOString().split('T')[0]
  } else {
    paymentDate.value = ''
  }
})

const confirm = () => {
  if (paymentDate.value) {
    emit('confirm', paymentDate.value)
  }
}

const cancel = () => {
  emit('cancel')
}
</script>