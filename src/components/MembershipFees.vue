<template>
  <div>
    <div class="sm:flex sm:items-center">
      <div class="sm:flex-auto">
        <h1 class="text-3xl font-bold text-gray-900">Jäsenmaksut</h1>
        <p class="mt-2 text-sm text-gray-700">
          Määrittele jäsenmaksujen hinnat vuosittain
        </p>
      </div>
      <div class="mt-4 sm:mt-0 sm:ml-16 sm:flex-none">
        <button
          @click="openAddModal"
          type="button"
          class="btn btn-primary"
        >
          Lisää jäsenmaksu
        </button>
      </div>
    </div>

    <!-- Suodattimet -->
    <div class="form-card">
      <div class="grid grid-cols-1 gap-4 sm:grid-cols-3">
        <div>
          <label class="form-label">Vuosi</label>
          <select
            v-model="filterYear"
            class="form-select"
          >
            <option value="">Kaikki vuodet</option>
            <option v-for="year in years" :key="year" :value="year">{{ year }}</option>
          </select>
        </div>
        <div>
          <label class="form-label">Jäsentyyppi</label>
          <select
            v-model="filterType"
            class="form-select"
          >
            <option value="">Kaikki tyypit</option>
            <option value="varsinainen">Varsinainen</option>
            <option value="nuorisojasen">Nuorisojasen</option>
            <option value="kannatus">Kannatus</option>
            <option value="kunnia">Kunnia</option>
          </select>
        </div>
      </div>
    </div>

    <!-- Jäsenmaksutaulukko -->
    <div class="data-table">
      <table class="min-w-full divide-y divide-gray-200">
        <thead class="bg-gray-50">
          <tr>
            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
              Vuosi
            </th>
            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
              Jäsentyyppi
            </th>
            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
              Summa
            </th>
            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
              Luotu
            </th>
            <th class="relative px-6 py-3">
              <span class="sr-only">Toiminnot</span>
            </th>
          </tr>
        </thead>
        <tbody class="bg-white divide-y divide-gray-200">
          <tr v-for="fee in filteredFees" :key="fee.id">
            <td class="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900">
              {{ fee.vuosi }}
            </td>
            <td class="px-6 py-4 whitespace-nowrap">
              <span
                class="badge"
                :class="getMemberTypeClass(fee.jasentyyppi)"
              >
                {{ getMemberTypeLabel(fee.jasentyyppi) }}
              </span>
            </td>
            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
              {{ formatCurrency(fee.summa) }}
            </td>
            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
              {{ formatDate(fee.created_at) }}
            </td>
            <td class="px-6 py-4 whitespace-nowrap text-right text-sm font-medium">
              <button
                @click="editFee(fee)"
                class="btn btn-sm btn-outline mr-2"
              >
                Muokkaa
              </button>
              <button
                @click="deleteFee(fee)"
                class="btn btn-sm btn-danger"
              >
                Poista
              </button>
            </td>
          </tr>
        </tbody>
      </table>
      
      <div v-if="filteredFees.length === 0" class="text-center py-8">
        <p class="text-gray-500">Ei jäsenmaksuja määritelty</p>
      </div>
    </div>

    <!-- Lisää/Muokkaa jäsenmaksu -modaali -->
    <div
      v-if="showModal"
      class="modal-overlay"
      @click="closeModal"
    >
      <div
        class="modal-content"
        @click.stop
      >
        <div class="mt-3">
          <h3 class="text-lg font-medium text-gray-900 mb-4">
            {{ editingFee ? 'Muokkaa jäsenmaksua' : 'Lisää uusi jäsenmaksu' }}
          </h3>
          
          <form @submit.prevent="saveFee" class="space-y-4">
            <div class="grid grid-cols-1 gap-4 sm:grid-cols-2">
              <div>
                <label class="form-label">Vuosi *</label>
                <select
                  v-model="feeForm.vuosi"
                  required
                  :disabled="!!editingFee"
                  class="form-select"
                  :class="{'opacity-50': !!editingFee}"
                >
                  <option v-for="year in years" :key="year" :value="year">{{ year }}</option>
                </select>
              </div>
              <div>
                <label class="form-label">Jäsentyyppi *</label>
                <select
                  v-model="feeForm.jasentyyppi"
                  required
                  :disabled="!!editingFee"
                  class="form-select"
                  :class="{'opacity-50': !!editingFee}"
                >
                  <option value="varsinainen">Varsinainen</option>
                  <option value="nuorisojasen">Nuorisojasen</option>
                  <option value="kannatus">Kannatus</option>
                  <option value="kunnia">Kunnia</option>
                </select>
              </div>
            </div>
            
            <div>
              <label class="form-label">Summa (€) *</label>
              <input
                v-model.number="feeForm.summa"
                type="number"
                step="0.01"
                min="0"
                required
                class="form-input"
              />
            </div>
            
            <div v-if="!editingFee" class="bg-blue-50 border border-blue-200 rounded-md p-4">
              <div class="flex">
                <div class="flex-shrink-0">
                  <svg class="h-5 w-5 text-blue-400" viewBox="0 0 20 20" fill="currentColor">
                    <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z" clip-rule="evenodd" />
                  </svg>
                </div>
                <div class="ml-3">
                  <h3 class="text-sm font-medium text-blue-800">
                    Huomio
                  </h3>
                  <div class="mt-2 text-sm text-blue-700">
                    <p>Voit määritellä saman vuoden jäsenmaksun vain kerran kullekin jäsentyypille.</p>
                  </div>
                </div>
              </div>
            </div>
            
            <div class="flex justify-end space-x-3 pt-4">
              <button
                type="button"
                @click="closeModal"
                class="btn btn-secondary"
              >
                Peruuta
              </button>
              <button
                type="submit"
                class="btn btn-primary"
              >
                {{ editingFee ? 'Tallenna' : 'Lisää' }}
              </button>
            </div>
          </form>
        </div>
      </div>
    </div>

    <!-- Varmistusdialogie -->
    <ConfirmDialog
      :show="confirmDialog.show"
      :title="confirmDialog.title"
      :message="confirmDialog.message"
      :type="confirmDialog.type"
      :icon="confirmDialog.icon"
      :confirm-text="confirmDialog.confirmText"
      :cancel-text="confirmDialog.cancelText"
      @confirm="confirmDialog.onConfirm"
      @cancel="closeConfirmDialog"
    />

    <!-- Virhe dialogi -->
    <AlertDialog
      :show="showErrorDialog"
      title="Virhe"
      :message="errorMessage"
      type="error"
      icon="error"
      @close="showErrorDialog = false"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import AlertDialog from './AlertDialog.vue'
import ConfirmDialog from './ConfirmDialog.vue'
import { formatDate } from '../utils/dateUtils'

interface MembershipFee {
  id: number
  vuosi: number
  jasentyyppi: string
  summa: number
  created_at: string
}

const fees = ref<MembershipFee[]>([])
const filterYear = ref('')
const filterType = ref('')
const showModal = ref(false)
const editingFee = ref<MembershipFee | null>(null)
const showErrorDialog = ref(false)
const errorMessage = ref('')

const confirmDialog = ref({
  show: false,
  title: '',
  message: '',
  type: 'warning' as 'warning' | 'danger' | 'info',
  icon: 'warning' as 'warning' | 'danger' | 'info',
  confirmText: 'Kyllä',
  cancelText: 'Peruuta',
  onConfirm: () => {}
})

const years = computed(() => {
  const currentYear = new Date().getFullYear()
  return Array.from({ length: 10 }, (_, i) => currentYear - 5 + i)
})

const feeForm = ref({
  vuosi: new Date().getFullYear(),
  jasentyyppi: 'varsinainen',
  summa: 0,
})

const filteredFees = computed(() => {
  return fees.value.filter(fee => {
    const matchesYear = !filterYear.value || fee.vuosi.toString() === filterYear.value
    const matchesType = !filterType.value || fee.jasentyyppi === filterType.value
    
    return matchesYear && matchesType
  }).sort((a, b) => {
    // Lajittele ensin vuoden mukaan (uusin ensin), sitten jäsentyypin mukaan
    if (a.vuosi !== b.vuosi) {
      return b.vuosi - a.vuosi
    }
    return a.jasentyyppi.localeCompare(b.jasentyyppi)
  })
})

const getMemberTypeClass = (type: string) => {
  switch (type) {
    case 'varsinainen':
      return 'bg-blue-100 text-blue-800'
    case 'nuorisojasen':
      return 'bg-yellow-100 text-yellow-800'
    case 'kannatus':
      return 'bg-green-100 text-green-800'
    case 'kunnia':
      return 'bg-purple-100 text-purple-800'
    default:
      return 'bg-gray-100 text-gray-800'
  }
}

const getMemberTypeLabel = (type: string) => {
  switch (type) {
    case 'varsinainen':
      return 'Varsinainen'
    case 'nuorisojasen':
      return 'Nuorisojasen'
    case 'kannatus':
      return 'Kannatus'
    case 'kunnia':
      return 'Kunnia'
    default:
      return type
  }
}

const formatCurrency = (amount: number) => {
  return new Intl.NumberFormat('fi-FI', {
    style: 'currency',
    currency: 'EUR'
  }).format(amount)
}


const openAddModal = () => {
  editingFee.value = null
  feeForm.value = {
    vuosi: new Date().getFullYear(),
    jasentyyppi: 'varsinainen',
    summa: 0,
  }
  showModal.value = true
}

const editFee = (fee: MembershipFee) => {
  editingFee.value = fee
  feeForm.value = {
    vuosi: fee.vuosi,
    jasentyyppi: fee.jasentyyppi,
    summa: fee.summa,
  }
  showModal.value = true
}

const closeModal = () => {
  showModal.value = false
  editingFee.value = null
}

const showConfirmDialog = (options: {
  title: string
  message: string
  type?: 'warning' | 'danger' | 'info'
  confirmText?: string
  cancelText?: string
  onConfirm: () => void
}) => {
  confirmDialog.value = {
    show: true,
    title: options.title,
    message: options.message,
    type: options.type || 'warning',
    icon: options.type || 'warning',
    confirmText: options.confirmText || 'Kyllä',
    cancelText: options.cancelText || 'Peruuta',
    onConfirm: () => {
      options.onConfirm()
      closeConfirmDialog()
    }
  }
}

const closeConfirmDialog = () => {
  confirmDialog.value.show = false
}

const saveFee = async () => {
  try {
    // Muunna jäsentyyppi backend:in odottamaan muotoon
    const feeData = {
      ...feeForm.value,
      jasentyyppi: feeForm.value.jasentyyppi.charAt(0).toUpperCase() + feeForm.value.jasentyyppi.slice(1) // varsinainen -> Varsinainen
    }
    
    if (editingFee.value) {
      await invoke('update_membership_fee', { 
        id: editingFee.value.id, 
        fee: feeData
      })
    } else {
      await invoke('create_membership_fee', { fee: feeData })
    }
    await loadFees()
    closeModal()
  } catch (error) {
    console.error('Virhe tallentaessa jäsenmaksua:', error)
    errorMessage.value = 'Virhe tallentaessa jäsenmaksua: ' + error
    showErrorDialog.value = true
  }
}

const deleteFee = async (fee: MembershipFee) => {
  // Kysy varmistus ENNEN poistamista
  const confirmMessage = `Haluatko varmasti poistaa jäsenmaksun:\n\nVuosi: ${fee.vuosi}\nJäsentyyppi: ${getMemberTypeLabel(fee.jasentyyppi)}\nSumma: ${formatCurrency(fee.summa)}\n\nHuom: Tämä ei vaikuta jo luotuihin laskuihin.`
  
  showConfirmDialog({
    title: 'Poista jäsenmaksu',
    message: confirmMessage,
    type: 'danger',
    confirmText: 'Poista',
    cancelText: 'Peruuta',
    onConfirm: async () => {
      try {
        await invoke('delete_membership_fee', { id: fee.id })
        await loadFees()
      } catch (error) {
        console.error('Virhe poistaessa jäsenmaksua:', error)
        errorMessage.value = 'Virhe poistaessa jäsenmaksua'
      showErrorDialog.value = true
      }
    }
  })
}

const loadFees = async () => {
  try {
    fees.value = await invoke('get_membership_fees')
  } catch (error) {
    console.error('Virhe ladatessa jäsenmaksuja:', error)
    fees.value = []
  }
}

onMounted(() => {
  loadFees()
})
</script>